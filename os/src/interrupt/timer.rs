use crate::sbi::set_timer;
use riscv::register::{time, sie};

static INTERVAL: usize = 100000;

pub fn init() {
    unsafe {
        //就是这句话，让我找了好久，MD
        //sstatus::set_sie();
        sie::set_stimer();
    }
    set_next_timeout();
}


pub fn set_next_timeout() {
    set_timer(time::read() + INTERVAL);
}

pub fn get_time() -> usize {
    time::read()
}