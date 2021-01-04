use crate::dispatch::*;
use crate::fs::*;
use alloc::sync::Arc;
use core::slice::from_raw_parts_mut;

pub fn sys_pipe(fd: *mut usize) -> isize{
    let process = SCHEDULER.lock().current_process().unwrap().clone();
    let fd = unsafe { from_raw_parts_mut(fd, 2) };
    let (read, write) = Pipe::create_pair();
    let readfd = process.inner().descriptors.len();
    process.inner().descriptors.push(Arc::new(read));
    let writefd = process.inner().descriptors.len();
    process.inner().descriptors.push(Arc::new(write));
    fd[0] = readfd as usize;
    fd[1] = writefd as usize;
    0
}