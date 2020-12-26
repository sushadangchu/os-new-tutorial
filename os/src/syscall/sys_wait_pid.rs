use crate::dispatch::*;

pub fn sys_wait_pid(pid: isize) -> isize {
    let child = SCHEDULER.lock().current_process().unwrap().inner().child.clone();
    //等待所有子进程
    if pid == -1 {
        for i in child.iter() {
            if SCHEDULER.lock().find_process_id(*i) == true {
                return -2;
            }
        }
        return -1;
    }
    0
}