use crate::process::next_app;
use super::SYSCALL_YIELD;

pub fn sys_yield() -> ! {
    //println!("[S] yield (:");
    next_app(SYSCALL_YIELD)
}