use crate::process::*;
use crate::dispatch::*;

pub fn sys_fork() -> isize {
    let process = SCHEDULER.lock().current_process().unwrap();
    
    let new_process = process.fork();
    let pid = process.id;
    // let context = process.get_context();
    // unsafe {
    //     (*context).x[10] = 0;
    // }
    //let process = Arc::new(process);
    SCHEDULER.lock().add_process(new_process);
    pid
}