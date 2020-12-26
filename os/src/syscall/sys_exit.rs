use crate::process::next_app;
use super::SYSCALL_EXIT;

pub fn sys_exit(_arg0: usize) -> ! {
    println!("[K] exit (:");
    next_app(SYSCALL_EXIT)
}