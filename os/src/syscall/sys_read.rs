use core::slice::from_raw_parts_mut;
use crate::dispatch::*;
use crate::process::next_app;
use super::SYSCALL_YIELD;

pub fn sys_read(fd: usize, buf: *mut u8, len: usize) -> isize {
    let process = SCHEDULER.lock().current_process().unwrap().clone();
    if let Some(inode) = process.inner().descriptors.get(fd) {
        let buffer = unsafe { from_raw_parts_mut(buf, len) };
        if let Ok(ret) = inode.read_at(0, buffer) {
            let ret = ret as isize;
            if ret >= 0 {
                return ret;
            }
        }
    }
    -1
}