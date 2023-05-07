use crate::task::task_manager::translate_cur_byte_buffer;

// buf pointer is an address in user space
// but now satp is kernel satp
// translate the user addr into kernel addr...
pub fn sys_write_impl(fd: usize, buf: *const u8, len: usize) -> isize {
    if fd != 1 {
        panic!("Not support for other file descriptor!");
    }
    if let Some(bufs) = translate_cur_byte_buffer(buf as usize, len) {
        for slice in bufs {
            print!("{}", core::str::from_utf8(slice).unwrap());
        }
        len as isize
    } else {
        panic!("Can not find the write buffer's physical address");
    }
}
