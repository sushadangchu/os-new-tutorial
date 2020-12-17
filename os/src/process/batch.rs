use crate::dispatch::*;

pub fn next_app(sys_id: usize) -> !{
    extern "C" {
        fn __restore(context: usize);
    }
    let context_ptr = SCHEDULER.get_ptr(sys_id);
    if SCHEDULER.get_app_num() == 0 {
        panic!("[S] all app end ");
    }
    unsafe {
        __restore(context_ptr)
    };

    panic!("batch end!");
}