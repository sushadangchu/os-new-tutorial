use crate::dispatch::*;
use core::slice::from_raw_parts_mut;

pub fn sys_write(fd: usize, buf: *mut u8, len: usize) -> isize{
    let process = SCHEDULER.lock().current_process().unwrap().clone();
    if let Some(inode) = process.inner().descriptors.get(fd) {
        let buffer = unsafe { from_raw_parts_mut(buf, len) };
        if let Ok(ret) = inode.write_at(0, buffer) {
            let ret = ret as isize;
            if ret >= 0 {
                return ret;
            }
        }
    }
    return -1;
}