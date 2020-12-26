use crate::process::*;
use super::*;
use crate::fs::*;
use crate::loader::get_app_data_by_name;
use crate::dispatch::*;
use alloc::sync::Arc;

pub fn sys_exec(buf: *const u8, len: usize) -> isize{
    let slice = unsafe { core::slice::from_raw_parts(buf, len) };
    let str = core::str::from_utf8(slice).unwrap();
    let app = ROOT_INODE.find(str).unwrap();
    let elf_data = app.readall().unwrap();
    SCHEDULER.lock().current_process().unwrap().exec(elf_data.as_slice());
    0
}