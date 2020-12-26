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
mod dispatch;
mod memory;
mod drivers;
mod fs;

extern crate alloc;
use crate::memory::*;
use crate::process::*;
use crate::dispatch::*;
use alloc::sync::Arc;
use crate::fs::*;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub extern "C" fn rust_main(_hart_id: usize, dtb_pa: PhysicalAddress) {
    memory::init();
    interrupt::init();
    drivers::init(dtb_pa);
    fs::init();

    let app = fs::ROOT_INODE.find("hello_world").unwrap();
    let data = app.readall().unwrap();
    let process1 = Arc::new(Process::new(data.as_slice()));
    //let process1 = Arc::new(Process::new(get_app_data_by_name("hello_world").unwrap()));
    //let process2 = Arc::new(Process::new(get_app_data_by_name("01power_5").unwrap()));
    //let process3 = Arc::new(Process::new(get_app_data_by_name("02power_7").unwrap()));
    // let process4 = Arc::new(Process::new(get_app_data_by_name("03sleep").unwrap()));

    SCHEDULER.lock().add_process(process1);
    //SCHEDULER.lock().add_process(process2);
    //SCHEDULER.lock().add_process(process3);
    // SCHEDULER.lock().add_process(process4);

    process::next_app(0);
    
    //panic!("end of rustmain")
}