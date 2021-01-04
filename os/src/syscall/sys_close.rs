use crate::dispatch::*;

pub fn sys_close(fd: usize) -> isize{
    let process = SCHEDULER.lock().current_process().unwrap().clone();
    process.inner().descriptors.remove(fd);
    0
}