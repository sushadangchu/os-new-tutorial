mod sys_exit;
mod sys_write;
mod sys_yield;
mod sys_get_time;
mod sys_get_pid;
mod sys_exec;
mod sys_fork;
mod sys_wait_pid;
mod sys_read;
mod sys_pipe;
mod sys_close;

const SYSCALL_CLOSE: usize = 57;
const SYSCALL_PIPE: usize = 59;
const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
pub const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_GET_PID: usize = 172;
const SYSCALL_FORK: usize = 220;
const SYSCALL_EXEC: usize = 221;
const SYSCALL_WAIT_PID: usize = 260;

use sys_write::*;
use sys_exit::*;
use sys_yield::*;
use sys_get_time::*;
use sys_get_pid::*;
use sys_exec::*;
use sys_fork::*;
use sys_wait_pid::*;
use sys_read::*;
use sys_pipe::*;
use sys_close::*;

pub fn syscall(which: usize, arg0: usize, arg1: usize, arg2: usize) -> isize {
    match which {
        SYSCALL_WRITE => sys_write(arg0, arg1 as *mut u8, arg2),
        SYSCALL_EXIT => sys_exit(arg0),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GET_TIME => sys_get_time() as isize,
        SYSCALL_GET_PID => sys_get_pid(),
        SYSCALL_EXEC => sys_exec(arg0 as *const u8, arg1),
        SYSCALL_FORK => sys_fork(),
        SYSCALL_WAIT_PID => sys_wait_pid(arg0 as isize),
        SYSCALL_READ => sys_read(arg0, arg1 as *mut u8, arg2),
        SYSCALL_PIPE => sys_pipe(arg0 as *mut usize),
        SYSCALL_CLOSE => sys_close(arg0),
        _ => panic!("无此系统调用!"),
    }
}