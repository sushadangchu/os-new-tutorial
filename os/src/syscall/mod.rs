mod sys_exit;
mod sys_write;

const SYS_WRITE: usize = 1;
const SYS_EXIT: usize = 2;

use sys_write::*;
use sys_exit::*;

pub fn syscall(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    match which {
        SYS_WRITE => sys_write(arg0, arg1 as *const u8, arg2),
        SYS_EXIT => sys_exit(arg0),
        _ => panic!("无此系统调用!"),
    }
}