use crate::process::next_app;

pub fn sys_exit(arg0: usize) -> ! {
    println!("[S] exit (:");
    next_app(2)
}