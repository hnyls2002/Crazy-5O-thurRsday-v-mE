use core::slice;

use alloc::vec::Vec;
use lazy_static::lazy_static;

pub struct AppData {
    pub name: &'static str,
    pub layout: (usize, usize),
}

fn app_data_init() -> Vec<AppData> {
    extern "C" {
        fn _num_app();
        fn _app_names();
    }
    unsafe {
        let app_num = (_num_app as *const usize).read_volatile();
        let app_layout_list = slice::from_raw_parts((_num_app as *const usize).add(1), app_num + 1);
        let mut name_ptr = _app_names as *const u8;
        let mut ret = Vec::new();
        for i in 0..app_num {
            let mut end = name_ptr;
            while end.read_volatile() != '\0' as u8 {
                end = end.add(1);
            }
            let slice = slice::from_raw_parts(name_ptr, end as usize - name_ptr as usize);
            let name_string = core::str::from_utf8(slice).expect("app name is not utf8");
            ret.push(AppData {
                name: name_string,
                layout: (app_layout_list[i], app_layout_list[i + 1]),
            });
            name_ptr = end.add(1);
        }
        // for i in 0..app_num {
        //     debug!("app_{}: {}", i, ret[i].name);
        // }
        ret
    }
}

lazy_static! {
    static ref APP_DATA: Vec<AppData> = app_data_init();
}

// get idx-th app's data
fn load_app(idx: usize) -> Option<&'static [u8]> {
    if idx >= APP_DATA.len() {
        return None;
    }
    unsafe {
        Some(slice::from_raw_parts(
            APP_DATA[idx].layout.0 as *const u8,
            APP_DATA[idx].layout.1 - APP_DATA[idx].layout.0,
        ))
    }
}

pub fn load_app_by_name(name: &str) -> Option<&'static [u8]> {
    (0..APP_DATA.len())
        .find(|&i| APP_DATA[i].name == name)
        .map_or(None, |idx| Some(load_app(idx)?))
}

pub fn get_app_names() -> Vec<&'static str> {
    let mut ret = Vec::new();
    for it in APP_DATA.iter() {
        ret.push(it.name)
    }
    ret
}
