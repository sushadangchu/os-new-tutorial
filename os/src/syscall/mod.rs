mod sys_exit;
mod sys_write;
mod sys_yield;
mod sys_get_time;

const SYSCALL_WRITE: usize = 64;
pub const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;

use sys_write::*;
use sys_exit::*;
use sys_yield::*;
use sys_get_time::*;

pub fn syscall(which: usize, arg0: usize, arg1: usize, arg2: usize) -> isize {
    match which {
        SYSCALL_WRITE => sys_write(arg0, arg1 as *const u8, arg2),
        SYSCALL_EXIT => sys_exit(arg0),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time() as isize,
        _ => panic!("无此系统调用!"),
    }
}