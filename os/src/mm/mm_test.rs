use crate::{
    info,
    mm::{kernel_space::kernel_pte, PTEFlags},
};

use super::VirtAddr;

pub fn remap_test() {
    extern "C" {
        pub fn stext();
        pub fn etext();
        pub fn srodata();
        pub fn erodata();
        pub fn sdata();
        pub fn edata();
        pub fn sbss();
        pub fn ebss();
    }
    info!("remap_test start!");

    let mid_text_vp = VirtAddr((stext as usize + etext as usize) >> 1).floor_page();
    let mid_rodata_vp = VirtAddr((srodata as usize + erodata as usize) >> 1).floor_page();
    let mid_data_vp = VirtAddr((sdata as usize + edata as usize) >> 1).floor_page();
    let mid_bss_vp = VirtAddr((ebss as usize + sbss as usize) >> 1).floor_page();

    assert!(
        kernel_pte(mid_text_vp)
            .expect("failed to find .text pte")
            .get_flags()
            == (PTEFlags::V | PTEFlags::R | PTEFlags::X),
        "text permission error"
    );

    assert!(
        kernel_pte(mid_rodata_vp)
            .expect("failed to find .rodata pte")
            .get_flags()
            == PTEFlags::V | PTEFlags::R,
        "rodata permission error"
    );

    assert!(
        kernel_pte(mid_data_vp)
            .expect("failed to find .data pte")
            .get_flags()
            == PTEFlags::V | PTEFlags::R | PTEFlags::W,
        "data permission error"
    );

    assert!(
        kernel_pte(mid_bss_vp)
            .expect("failed to find .bss pte")
            .get_flags()
            == PTEFlags::V | PTEFlags::R | PTEFlags::W,
        "bss permission error"
    );

    info!("remap_test passed!");
}
