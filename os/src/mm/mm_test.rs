use crate::{
    info,
    mm::{
        memory_set::{ebss, sbss},
        PTEFlags,
    },
};

use super::{
    memory_set::{edata, erodata, etext, sdata, srodata, stext},
    VirtAddr, KERNEL_SPACE,
};

pub fn remap_test() {
    info!("remap_test start!");

    let page_table = &KERNEL_SPACE.exclusive_access().page_table;
    let mid_text_vp = VirtAddr((stext as usize + etext as usize) >> 1).floor_page();
    let mid_rodata_vp = VirtAddr((srodata as usize + erodata as usize) >> 1).floor_page();
    let mid_data_vp = VirtAddr((sdata as usize + edata as usize) >> 1).floor_page();
    let mid_bss_vp = VirtAddr((ebss as usize + sbss as usize) >> 1).floor_page();

    assert!(
        page_table
            .find_pte(mid_text_vp)
            .expect("failed to find .text pte")
            .get_flags()
            == (PTEFlags::V | PTEFlags::R | PTEFlags::X),
        "text permission error"
    );

    assert!(
        page_table
            .find_pte(mid_rodata_vp)
            .expect("failed to find .rodata pte")
            .get_flags()
            == PTEFlags::V | PTEFlags::R,
        "rodata permission error"
    );

    assert!(
        page_table
            .find_pte(mid_data_vp)
            .expect("failed to find .data pte")
            .get_flags()
            == PTEFlags::V | PTEFlags::R | PTEFlags::W,
        "data permission error"
    );

    assert!(
        page_table
            .find_pte(mid_bss_vp)
            .expect("failed to find .bss pte")
            .get_flags()
            == PTEFlags::V | PTEFlags::R | PTEFlags::W,
        "bss permission error"
    );

    info!("remap_test passed!");
}
