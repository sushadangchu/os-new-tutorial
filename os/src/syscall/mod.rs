mod sys_exit;
mod sys_write;
mod sys_yield;
mod sys_get_time;

pub const SYS_WRITE: usize = 1;
pub const SYS_EXIT: usize = 2;
pub const SYS_YIELD: usize = 3;
pub const SYS_GET_TIME: usize = 4;

use sys_write::*;
use sys_exit::*;
use sys_yield::*;
use sys_get_time::*;

pub fn syscall(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    match which {
        SYS_WRITE => sys_write(arg0, arg1 as *const u8, arg2),
        SYS_EXIT => sys_exit(arg0),
        SYS_YIELD => sys_yield(),
        SYS_GET_TIME => sys_get_time(),
        _ => panic!("无此系统调用!"),
    }
}