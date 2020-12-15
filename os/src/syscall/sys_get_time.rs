use crate::interrupt::get_time;

pub fn sys_get_time() -> usize {
    get_time()
}