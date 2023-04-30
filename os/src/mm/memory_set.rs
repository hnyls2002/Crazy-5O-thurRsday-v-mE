use alloc::vec::Vec;
use lazy_static::lazy_static;
use riscv::register::satp;

use crate::kfc_util::up_safe_cell::UPSafeCell;

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

    pub fn get_pt_root_frame(&self) -> Frame {
        self.page_table.entry
    }

    /// build realations in **page_table**
    pub fn insert_new_map_area(&mut self, map_area: MapArea) {
        let vp_range = &map_area.vp_range;
        let pte_flags = PTEFlags::from_bits(map_area.map_perm.bits()).unwrap();
        // TODO : PTE flags may have other flags to be set

        for it in vp_range.iter() {
            let vp = it.value();
            let pp = map_area.get_framed(vp);
            let res = self.page_table.map_one(vp, pp, pte_flags);
            if let Err(_) = res {
                panic!("virtual page mapping to physical page failed");
            }
        }

        self.map_areas.push(map_area);
    }

    /// release the relations in **page_table**
    pub fn relase_area(&mut self, vp_range: &VPRange) {
        todo!()
    }
}

#[allow(unused)]
extern "C" {
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sbss_with_stack();
    fn sbss();
    fn ebss();
    fn skernel();
    fn ekernel();
    fn strampoline();
}

lazy_static! {
    static ref KERNEL_SPACE: UPSafeCell<MemorySet> = UPSafeCell::new(MemorySet::new_kernel_space());
}

impl MemorySet {
    pub fn new_kernel_space() -> Self {
        // TODO : trampoline to be set...
        let mut memory_set = MemorySet::new();

        // .text
        let text = MapArea::new_bare(
            VPRange::new(VirtAddr(stext as usize), VirtAddr(etext as usize)),
            MapType::Identical,
            MapPerm::R | MapPerm::X,
        );
        memory_set.insert_new_map_area(text);

        // .rodata
        let rodata = MapArea::new_bare(
            VPRange::new(VirtAddr(srodata as usize), VirtAddr(erodata as usize)),
            MapType::Identical,
            MapPerm::R,
        );
        memory_set.insert_new_map_area(rodata);

        // .data
        let data = MapArea::new_bare(
            VPRange::new(VirtAddr(sdata as usize), VirtAddr(edata as usize)),
            MapType::Identical,
            MapPerm::R | MapPerm::W,
        );
        memory_set.insert_new_map_area(data);

        // .bss (with stack)
        let bss = MapArea::new_bare(
            VPRange::new(VirtAddr(sbss_with_stack as usize), VirtAddr(ebss as usize)),
            MapType::Identical,
            MapPerm::R | MapPerm::W,
        );
        memory_set.insert_new_map_area(bss);

        // available physical frames
        let pool = MapArea::new_bare(
            VPRange::new(VirtAddr(skernel as usize), VirtAddr(ekernel as usize)),
            MapType::Identical,
            MapPerm::R | MapPerm::W,
        );
        memory_set.insert_new_map_area(pool);

        todo!()
    }
}

pub fn activate_kernel_space() {
    let ppn = KERNEL_SPACE
        .exclusive_access()
        .get_pt_root_frame()
        .get_ppn();
    unsafe { satp::set(satp::Mode::Sv39, 0, ppn) };
    // TODO : asid
    // TODO : memory barrierj...
}
