use crate::process::*;
use crate::dispatch::*;

pub fn sys_get_pid() -> isize {
    SCHEDULER.lock().current_process().unwrap().get_pid()
}