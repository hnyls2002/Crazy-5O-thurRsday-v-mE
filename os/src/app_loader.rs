use core::slice;

use alloc::vec::Vec;
use lazy_static::lazy_static;

fn get_app_layout() -> Vec<(usize, usize)> {
    extern "C" {
        fn _num_app();
    }
    let app_num = unsafe { (_num_app as *const usize).read_volatile() };
    let app_layout_list =
        unsafe { slice::from_raw_parts((_num_app as *const usize).add(1), app_num + 1) };
    let mut ret = Vec::new();
    for i in 0..app_num {
        ret.push((app_layout_list[i], app_layout_list[i + 1]));
    }

    ret
}

lazy_static! {
    // pub static ref APP_NUM: usize = unsafe { (_num_app as *const usize).read_volatile() };
    static ref APP_LAYOUT_INFOS : Vec<(usize,usize)> = get_app_layout();
}

#[allow(unused)]
pub fn loader_debug() {
    debug!("The number of apps: {}", APP_LAYOUT_INFOS.len());
    for app_layout in APP_LAYOUT_INFOS.iter() {
        warn!(
            "app start: {:#X}, app end: {:#X}, size is {}KB",
            app_layout.0,
            app_layout.1,
            (app_layout.1 - app_layout.0) / 1024
        );
    }
}

// get idx-th app's data
pub fn load_app(idx: usize) -> &'static [u8] {
    assert!(idx < APP_LAYOUT_INFOS.len(), "app index out of range!");
    unsafe {
        slice::from_raw_parts(
            APP_LAYOUT_INFOS[idx].0 as *const u8,
            APP_LAYOUT_INFOS[idx].1 - APP_LAYOUT_INFOS[idx].0,
        )
    }
}

pub fn get_app_num() -> usize {
    APP_LAYOUT_INFOS.len()
}

#[allow(unused)]
pub fn get_app_address_range(idx: usize) -> (usize, usize) {
    assert!(idx < APP_LAYOUT_INFOS.len(), "app index out of range!");
    APP_LAYOUT_INFOS[idx]
}
