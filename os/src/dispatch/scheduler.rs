use lazy_static::*;
use core::cell::RefCell;
use alloc::vec::Vec;
use crate::process::*;
use crate::loader::*;
use crate::syscall::SYSCALL_EXIT;

pub struct Scheduler {
    pub inner: RefCell<SchedulerInner>,
}

pub struct SchedulerInner {
    pub processes: Vec<Process>,
    pub current_process: usize,
    pub app_num: usize,
}

unsafe impl Sync for Scheduler {}

lazy_static! {
    pub static ref SCHEDULER: Scheduler = {
        let app_num = get_app_num();
        
        let mut processes: Vec<Process> = Vec::new();
        
        for i in 0..app_num {
            processes.push(Process::new(get_app_data(i), i));
        }

        Scheduler {
            inner: RefCell::new(SchedulerInner {
                processes,
                current_process: 0,
                app_num,
            }),
        }
    };
}

impl Scheduler {
    pub fn get_app_num(&self) -> usize{
        self.inner.borrow().app_num
    }

    pub fn get_ptr(&self, sys_id: usize) -> usize{
        let mut inner = self.inner.borrow_mut();
        let total_app_num = get_app_num();
        let app_num = inner.app_num;
        let current_process = inner.current_process;
        let mut next_run = 0;
        let mut flag = false;
        for i in 1..=inner.processes.len() {
            let j = (i + current_process) % total_app_num;
            if inner.processes[j].state == ProcessStatus::Ready {
                next_run = j;
                flag = true;
                break;
            }
        }

        if sys_id == SYSCALL_EXIT {
            inner.app_num -= 1;
            println!("{}", inner.app_num);
            inner.processes[current_process].state = ProcessStatus::Exited;
        } else {
            inner.processes[current_process].state = ProcessStatus::Ready;
        }

        if flag == false {
            if app_num == 1 {
                return inner.processes[current_process].context_ptr;
            }
        }


        if inner.processes[current_process].state == ProcessStatus::Running {
            inner.processes[current_process].state = ProcessStatus::Ready;
        }
        
        inner.current_process = next_run;
        inner.processes[next_run].state = ProcessStatus::Running;
        inner.processes[next_run].memory_set.activate();
        inner.processes[next_run].context_ptr
    }
}