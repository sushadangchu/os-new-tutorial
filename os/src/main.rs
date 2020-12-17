#![no_std]

#![no_main]

#![feature(alloc_error_handler)]

#![feature(llvm_asm)]

#![feature(global_asm)]

#![feature(panic_info_message)]

#![feature(naked_functions)]

#![feature(slice_fill)]

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

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub extern "C" fn rust_main() {
    //interrupt::init();
    memory::init();
    interrupt::init();

    //第一个参数为开始运行的app，第二个参数为初始状态
    process::next_app(0);

    panic!("end of rustmain")
}