use core::cmp::max;

use alloc::{collections::BTreeMap, vec::Vec};

use crate::{
    config::{TRAMPOLINE_VIRT_ADDR, TRAP_CTX_VIRT_ADDR, USER_STACK_SIZE},
    mm::map_area::FillData,
};

use super::{Frame, MapArea, MapPerm, MapType, PTEFlags, PageTable, VPRange, VirtAddr};

pub struct MemorySet {
    pub map_areas: Vec<MapArea>,
    pub page_table: PageTable,
}

impl MemorySet {
    /// #### only page table root is set
    pub fn new_bare() -> Self {
        MemorySet {
            map_areas: Vec::new(),
            page_table: PageTable::new(),
        }
    }

    fn get_pt_root_frame(&self) -> Frame {
        self.page_table.entry
    }

    pub fn get_satp_token(&self) -> usize {
        8usize << 60 | self.get_pt_root_frame().get_ppn()
    }

    /// build realations in **page_table**
    pub fn insert_new_map_area(&mut self, map_area: MapArea) {
        let vp_range = &map_area.vp_range;
        let pte_flags = PTEFlags::from_bits(map_area.map_perm.bits()).unwrap();
        // TODO : PTE flags may have other flags to be set

        // trace!("insert new map area : {:#X?}", map_area);
        for it in vp_range.iter() {
            let vp = it.value();
            let pp = map_area.mapped_to(vp);
            // trace!("vp={:#X?}, pp={:#X?}", vp, pp);
            let res = self.page_table.map_one(vp, pp, pte_flags);
            assert!(res.is_ok(), "virtual page mapping to physical page failed");
        }

        self.map_areas.push(map_area);
    }

    /// release the relations in **page_table**
    pub fn relase_area(&mut self, vp_range: &VPRange) {
        let index = self
            .map_areas
            .iter()
            .position(|ma| ma.vp_range == *vp_range)
            .expect("no such map area fitting the vp_range");

        // move the value out
        let map_area = self.map_areas.remove(index);

        // release the relations in page_table
        for it in map_area.vp_range.iter() {
            let vp = it.value();
            if let Err(_) = self.page_table.unmap_one(vp) {
                panic!("unmap a page failed")
            }
        }

        // the map_area will be dropped here
    }
}

impl MemorySet {
    /// return (`memory_set`, `entry_point`, `user_stack_top`)
    pub fn new_from_elf(elf_data: &[u8]) -> (Self, usize, usize) {
        let mut memory_set = MemorySet::new_bare();

        // insert trampoline
        memory_set.insert_new_map_area(MapArea::new_trampoline());

        // insert trap context
        let ctx_area = MapArea::new(
            VPRange::new(TRAP_CTX_VIRT_ADDR, TRAMPOLINE_VIRT_ADDR),
            MapType::Framed(BTreeMap::new()),
            MapPerm::R | MapPerm::W,
            None,
        );
        memory_set.insert_new_map_area(ctx_area);

        // parse elf file by xmas_elf
        let elf = xmas_elf::ElfFile::new(elf_data).expect("failed to parse elf");

        // get elf header
        let elf_headr = elf.header;

        // check if the magic is "0x7F E L F"
        let magic = elf_headr.pt1.magic;
        assert_eq!(
            magic,
            [0x7F, 'E' as u8, 'L' as u8, 'F' as u8],
            "elf magic error"
        );

        // get program header table
        let ph_count = elf.header.pt2.ph_count();
        let mut max_end_va = VirtAddr(0);

        // map all loadable segments
        for i in 0..ph_count {
            let ph = elf.program_header(i).expect("failed to get program header");
            // If this header is a loadable segment, map it into memory.
            if ph.get_type().unwrap() == xmas_elf::program::Type::Load {
                let start_va = VirtAddr(ph.virtual_addr() as usize);
                let end_va = start_va.step_offset(ph.mem_size() as usize);
                max_end_va = max(max_end_va, end_va);

                // map it with U permission and R/W/X according to the flags
                let mut map_perm = MapPerm::U;
                let ph_flag = ph.flags();
                if ph_flag.is_read() {
                    map_perm |= MapPerm::R;
                }
                if ph_flag.is_write() {
                    map_perm |= MapPerm::W;
                }
                if ph_flag.is_execute() {
                    map_perm |= MapPerm::X;
                }

                // build a map_area and bound frames
                let fill_data = FillData::new(
                    start_va,
                    start_va.step_offset(ph.file_size() as usize),
                    &elf.input[ph.offset() as usize..(ph.offset() + ph.file_size()) as usize],
                );

                let map_area = MapArea::new(
                    VPRange::new(start_va, end_va),
                    MapType::Framed(BTreeMap::new()),
                    map_perm,
                    Some(fill_data),
                );

                // insert the map_area into memory_set
                memory_set.insert_new_map_area(map_area);
            }
        }

        // build the user stack : next_page() actually build a guard page...
        let user_stack_bottom = max_end_va.ceil_page().next_page().start_address();
        let user_stack_top = user_stack_bottom.step_offset(USER_STACK_SIZE);
        let user_stack = MapArea::new(
            VPRange::new(user_stack_bottom, user_stack_top),
            MapType::Framed(BTreeMap::new()),
            MapPerm::U | MapPerm::R | MapPerm::W,
            None,
        );
        memory_set.insert_new_map_area(user_stack);

        (
            memory_set,
            elf_headr.pt2.entry_point() as usize,
            user_stack_top.0,
        )
    }

    // fork all the areas except for trampoline : Target
    pub fn fork_memory_set(&self) -> Self {
        let mut memory_set = MemorySet::new_bare();

        for area in self.map_areas.iter() {
            let map_type = match area.map_type {
                MapType::Identical => MapType::Identical,
                MapType::Target(frame) => MapType::Target(frame),
                MapType::Framed(_) => MapType::Framed(BTreeMap::new()),
            };

            let new_area =
                MapArea::new(area.vp_range.clone(), map_type, area.map_perm.clone(), None);

            // when framed, copy data
            if let MapType::Framed(_) = new_area.map_type {
                for it in new_area.vp_range.iter() {
                    let vp = it.value();
                    let pp_src = self.page_table.translate_vp(vp).expect("no physical page");
                    let pp_dst = new_area.mapped_to(vp);
                    pp_dst
                        .get_bytes_array_mut()
                        .copy_from_slice(pp_src.get_bytes_array_mut())
                }
            }

            memory_set.insert_new_map_area(new_area);
        }

        memory_set
    }
}
