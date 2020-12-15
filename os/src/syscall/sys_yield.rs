use crate::process::next_app;

pub fn sys_yield() -> ! {
    println!("[S] yield (:");
    next_app(3)
}