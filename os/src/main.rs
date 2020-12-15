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
pub extern "C" fn rust_main() {
    interrupt::init();

    //第一个参数为开始运行的app，第二个参数为初始状态
    process::next_app(0);

    //panic!("end of rustmain")
}