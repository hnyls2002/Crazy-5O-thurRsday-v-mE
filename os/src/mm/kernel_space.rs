use core::arch::asm;
use lazy_static::lazy_static;
use riscv::register::satp;

use crate::{
    config::MEMORY_END,
    kfc_sbi::mmio::MMIO,
    kfc_util::up_safe_cell::UPSafeCell,
    mm::{MapArea, MapPerm, MapType, VPRange, VirtAddr},
};

use super::{Frame, MemorySet, PageTable};

pub struct KernelSpace {
    inner: UPSafeCell<MemorySet>,
}

lazy_static! {
    pub static ref KERNEL_SPACE: KernelSpace = KernelSpace {
        inner: UPSafeCell::new(MemorySet::new_bare()),
    };
}

impl MemorySet {
    fn kernel_init(&mut self) {
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
            pub fn ekernel();
            pub fn strampoline();
        }

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

        self.insert_new_map_area(MapArea::new_trampoline());

        // .text
        let text = MapArea::new(
            VPRange::new(VirtAddr(stext as usize), VirtAddr(etext as usize)),
            MapType::Identical,
            MapPerm::R | MapPerm::X,
            None,
        );
        self.insert_new_map_area(text);

        // .rodata
        let rodata = MapArea::new(
            VPRange::new(VirtAddr(srodata as usize), VirtAddr(erodata as usize)),
            MapType::Identical,
            MapPerm::R,
            None,
        );
        self.insert_new_map_area(rodata);

        // .data
        let data = MapArea::new(
            VPRange::new(VirtAddr(sdata as usize), VirtAddr(edata as usize)),
            MapType::Identical,
            MapPerm::R | MapPerm::W,
            None,
        );
        self.insert_new_map_area(data);

        // .bss (with stack)
        let bss = MapArea::new(
            VPRange::new(VirtAddr(sbss_with_stack as usize), VirtAddr(ebss as usize)),
            MapType::Identical,
            MapPerm::R | MapPerm::W,
            None,
        );
        // debug!("insert bss into kernel space");
        self.insert_new_map_area(bss);
        // trace!("after bss");

        // available physical frames
        let pool = MapArea::new(
            VPRange::new(VirtAddr(ekernel as usize), VirtAddr(MEMORY_END as usize)),
            MapType::Identical,
            MapPerm::R | MapPerm::W,
            None,
        );
        // trace!("insert pool into kernel space");
        self.insert_new_map_area(pool);

        // MMIO
        for vp_range in MMIO {
            let ma = MapArea::new(vp_range, MapType::Identical, MapPerm::R | MapPerm::W, None);
            self.insert_new_map_area(ma);
        }
    }
}

impl KernelSpace {
    pub fn pt_entry(&self) -> Frame {
        self.inner.exclusive_access().page_table.entry
    }

    pub fn add_kernel_stack(&self, stack: MapArea) {
        self.inner.exclusive_access().insert_new_map_area(stack);
    }

    pub fn remove_map_area(&self, vp_range: &VPRange) {
        self.inner.exclusive_access().relase_area(vp_range);
    }
}

pub fn kernel_space_init() {
    KERNEL_SPACE.inner.exclusive_access().kernel_init()
}

pub fn activate_kernel_space() {
    let token = PageTable::satp_token(KERNEL_SPACE.pt_entry());
    unsafe {
        satp::write(token);
        // satp::set(satp::Mode::Sv39, 0, ppn);
        asm!("sfence.vma");
    };
}
