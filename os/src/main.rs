#![no_std]

#![no_main]

#![feature(llvm_asm)]

#![feature(global_asm)]

#![feature(panic_info_message)]

#[macro_use]
mod console;
mod panic;
mod sbi;
mod interrupt;
mod process;
mod syscall;
mod user;
mod dispatch;

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub extern "C" fn rust_main() -> !{
    interrupt::init();

    process::next_app(0);

    //panic!("end of rustmain")
}