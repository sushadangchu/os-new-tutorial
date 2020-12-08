#![no_std]

#![no_main]

#![feature(llvm_asm)]

#![feature(global_asm)]

#![feature(panic_info_message)]

#[macro_use]
mod console;
mod panic;
mod sbi;
mod syscall;
mod user_app;

global_asm!(include_str!("entry.asm"));

use crate::user_app::*;

#[no_mangle]
pub extern "C" fn rust_main() -> !{
    hello_world();
    let a = [1, 2, 3, 4, 5];
    count_sum(a);
    panic!("end of rustmain")
}