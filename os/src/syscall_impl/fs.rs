use crate::{console::console_getc, task::processor::translate_cur_byte_buffer_mut};

// buf pointer is an address in user space
// but now satp is kernel satp
// translate the user addr into kernel addr...
pub fn sys_write_impl(fd: usize, buf: *const u8, len: usize) -> isize {
    if fd != 1 {
        panic!("Not support for other file descriptor!");
    }
    if let Some(bufs) = translate_cur_byte_buffer_mut(buf as usize, len) {
        for slice in bufs {
            print!("{}", core::str::from_utf8(slice).unwrap());
        }
        len as isize
    } else {
        panic!("Can not find the write buffer's physical address");
    }
}

pub fn sys_read_impl(fd: usize, buf: *mut u8, len: usize) -> isize {
    if fd != 0 {
        panic!("Not support for other file descriptor!");
    }

    assert_eq!(len, 1, "Only support read one byte");

    let mut c: u8;
    loop {
        c = console_getc();
        if c != 0 {
            break;
        }
        // maybe we can yield here...
    }
    // write to the current task's address space
    if let Some(mut bufs) = translate_cur_byte_buffer_mut(buf as usize, len) {
        unsafe { bufs[0].as_mut_ptr().write_volatile(c) }
    }
    1
}
