use crate::process::next_app;

pub fn sys_exit(_arg0: usize) -> ! {
    println!("[S] exit (:");
    next_app(2)
}