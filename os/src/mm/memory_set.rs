use core::{arch::asm, cmp::max};

use alloc::{collections::BTreeMap, vec::Vec};
use lazy_static::lazy_static;
use riscv::register::satp;

use crate::{
    config::{MEMORY_END, TRAMPOLINE_VIRT_ADDR, TRAP_CTX_VIRT_ADDR, USER_STACK_SIZE},
    info,
    kfc_sbi::mmio::MMIO,
    kfc_util::up_safe_cell::UPSafeCell,
    mm::map_area::FillData,
};

use super::{Frame, MapArea, MapPerm, MapType, PTEFlags, PageTable, VPRange, VirtAddr};

pub struct MemorySet {
    pub map_areas: Vec<MapArea>,
    pub page_table: PageTable,
}

impl MemorySet {
    /// #### only page table root is set
    pub fn new() -> Self {
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
            let pp = map_area.get_framed(vp);
            // trace!("vp={:#X?}, pp={:#X?}", vp, pp);
            let res = self.page_table.map_one(vp, pp, pte_flags);
            assert!(res.is_ok(), "virtual page mapping to physical page failed");
        }

        self.map_areas.push(map_area);
    }

    #[allow(unused)]
    /// release the relations in **page_table**
    pub fn relase_area(&mut self, vp_range: &VPRange) {
        todo!()
    }
}

#[allow(unused)]
extern "C" {
    pub fn stext();
    pub fn etext();
    pub fn srodata();
    pub fn erodata();
    pub fn sdata();
    pub fn edata();
    pub fn sbss_with_stack();
    pub fn sbss();
    pub fn ebss();
    pub fn skernel();
    pub fn ekernel();
    pub fn strampoline();
}

lazy_static! {
    pub static ref KERNEL_SPACE: UPSafeCell<MemorySet> =
        UPSafeCell::new(MemorySet::new_kernel_space());
    pub static ref KERNEL_SATP: usize = KERNEL_SPACE.exclusive_access().get_satp_token();
}

impl MemorySet {
    pub fn new_kernel_space() -> Self {
        let mut memory_set = MemorySet::new();

        info!("-----------------------kernel space-----------------------");
        info!(
            ".text\t\t\t\t[{:#X?}, {:#X?})",
            stext as usize, etext as usize
        );
        info!(
            ".rodata\t\t\t\t[{:#X?}, {:#X?})",
            srodata as usize, erodata as usize
        );
        info!(
            ".data\t\t\t\t[{:#X?}, {:#X?})",
            sdata as usize, edata as usize
        );
        info!(".bss\t\t\t\t[{:#X?}, {:#X?})", sbss as usize, ebss as usize);
        info!(
            "frame pool\t\t\t[{:#X?}, {:#X?})",
            ekernel as usize, MEMORY_END as usize
        );
        info!(
            "trampoline\t\t\t[{:#X?}, {:#X?})",
            strampoline as usize,
            strampoline as usize + 0x1000
        );
        info!("-----------------------kernel space-----------------------");

        memory_set.insert_new_map_area(MapArea::new_trampoline());

        // .text
        let text = MapArea::new(
            VPRange::new(VirtAddr(stext as usize), VirtAddr(etext as usize)),
            MapType::Identical,
            MapPerm::R | MapPerm::X,
            None,
        );
        memory_set.insert_new_map_area(text);

        // .rodata
        let rodata = MapArea::new(
            VPRange::new(VirtAddr(srodata as usize), VirtAddr(erodata as usize)),
            MapType::Identical,
            MapPerm::R,
            None,
        );
        memory_set.insert_new_map_area(rodata);

        // .data
        let data = MapArea::new(
            VPRange::new(VirtAddr(sdata as usize), VirtAddr(edata as usize)),
            MapType::Identical,
            MapPerm::R | MapPerm::W,
            None,
        );
        memory_set.insert_new_map_area(data);

        // .bss (with stack)
        let bss = MapArea::new(
            VPRange::new(VirtAddr(sbss_with_stack as usize), VirtAddr(ebss as usize)),
            MapType::Identical,
            MapPerm::R | MapPerm::W,
            None,
        );
        // debug!("insert bss into kernel space");
        memory_set.insert_new_map_area(bss);
        // trace!("after bss");

        // available physical frames
        let pool = MapArea::new(
            VPRange::new(VirtAddr(ekernel as usize), VirtAddr(MEMORY_END as usize)),
            MapType::Identical,
            MapPerm::R | MapPerm::W,
            None,
        );
        // trace!("insert pool into kernel space");
        memory_set.insert_new_map_area(pool);

        // MMIO
        for vp_range in MMIO {
            let ma = MapArea::new(vp_range, MapType::Identical, MapPerm::R | MapPerm::W, None);
            memory_set.insert_new_map_area(ma);
        }
        memory_set
    }
}

pub fn activate_kernel_space() {
    let token = KERNEL_SPACE.exclusive_access().get_satp_token();

    unsafe {
        satp::write(token);
        // satp::set(satp::Mode::Sv39, 0, ppn);
        asm!("sfence.vma");
    };
}

impl MemorySet {
    /// return (`memory_set`, `entry_point`, `user_stack_top`)
    pub fn new_from_elf(elf_data: &[u8]) -> (Self, usize, usize) {
        let mut memory_set = MemorySet::new();

        memory_set.insert_new_map_area(MapArea::new_trampoline());

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
                let end_va = VirtAddr(ph.virtual_addr() as usize + ph.mem_size() as usize);
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
                    VirtAddr(start_va.0 + ph.file_size() as usize),
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
        let user_stack_top = VirtAddr(user_stack_bottom.0 + USER_STACK_SIZE);
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
}
