#![no_std]

#![no_main]

#![feature(alloc_error_handler)]

#![feature(llvm_asm)]

#![feature(global_asm)]

#![feature(panic_info_message)]

#![feature(naked_functions)]

#![feature(slice_fill)]

#![feature(drain_filter)]

#[macro_use]
mod console;
mod panic;
mod sbi;
mod interrupt;
mod process;
mod syscall;
mod user;
mod dispatch;
mod memory;
mod loader;

extern crate alloc;

use crate::loader::*;
use crate::process::*;
use crate::dispatch::*;
use alloc::sync::Arc;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub extern "C" fn rust_main() {
    //interrupt::init();
    memory::init();
    interrupt::init();

    let process1 = Arc::new(Process::new(get_app_data_by_name("00power_3").unwrap()));
    let process2 = Arc::new(Process::new(get_app_data_by_name("01power_5").unwrap()));
    //let process3 = Arc::new(Process::new(get_app_data_by_name("02power_7").unwrap()));
    //let process4 = Arc::new(Process::new(get_app_data_by_name("03sleep").unwrap()));

    SCHEDULER.lock().add_process(process1);
    SCHEDULER.lock().add_process(process2);
    //SCHEDULER.lock().add_process(process3);
    //SCHEDULER.lock().add_process(process4);

    //第一个参数为开始运行的app，第二个参数为初始状态
    process::next_app(0);

    panic!("end of rustmain")
}