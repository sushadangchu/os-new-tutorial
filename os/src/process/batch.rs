use crate::dispatch::*;

pub fn next_app(sys_id: usize) -> !{
    
    extern "C" {
        fn __restore(context: usize);
    }
    let context_ptr = SCHEDULER.lock().get_ptr(sys_id);
    
    unsafe {
        __restore(context_ptr)
    };

    panic!("batch end!");
}