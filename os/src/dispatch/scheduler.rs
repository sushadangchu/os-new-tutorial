use lazy_static::*;
use core::cell::RefCell;
use alloc::sync::Arc;
use alloc::vec::Vec;
use crate::process::*;
use crate::loader::*;
use alloc::collections::LinkedList;
use crate::syscall::SYSCALL_EXIT;
use super::Lock;

#[derive(Default)]
pub struct Scheduler {
    pub processes: LinkedList<Arc<Process>>,
    pub current_process: Option<Arc<Process>>,
    pub app_num: usize,
}

unsafe impl Sync for Scheduler {}

lazy_static! {
    pub static ref SCHEDULER: Lock<Scheduler> = Lock::new(Scheduler::new());
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            processes: LinkedList::new(),
            current_process: None,
            app_num: 0,
        }
    }
    pub fn get_app_num(&self) -> usize{
        self.app_num
    }

    pub fn add_process(&mut self, process: Arc<Process>) {
        self.processes.push_back(process);
    }

    pub fn get_next(&mut self) -> Option<Arc<Process>> {
        if let Some(process) = self.processes.pop_front() {
            Some(process)
        } else {
            None
        }
    }

    pub fn remove_process(&mut self, process: &Arc<Process>) {
        let mut removed = self.processes.drain_filter(|p| p == process);
        assert!(removed.next().is_some() && removed.next().is_none());
    }

    pub fn get_ptr(&mut self, sys_id: usize) -> usize {
        if let Some(process) = self.get_next() {
            if sys_id != SYSCALL_EXIT && self.current_process != None{
                let current_process = self.current_process.as_ref().unwrap().clone();
                self.add_process(current_process);
            }
            let context_ptr = process.prepare();
            self.current_process = Some(process);
            context_ptr
            // Some(process) => {
            //     //let mut current_process = self.current_process.unwrap().clone();
            //     //self.add_process(current_process);
            //     let context_ptr = process.prepare();
            //     self.current_process = Some(process);
            //     context_ptr
            // }
            // None => {
            //     if sys_id == SYSCALL_EXIT {
            //         //所有进程都结束了
            //         self.current_process = None;
            //         panic!("[K] all app end");
            //     } else {
            //         self.current_process.unwrap().clone().prepare()
            //     }
            // }
        } else {
            if sys_id == SYSCALL_EXIT {
                //所有进程都结束了
                //self.current_process = None;
                println!("hello");
                panic!("[K] all app end");
            } else {
              self.current_process.as_ref().unwrap().context_ptr
            }
        }
    }

    // pub fn get_ptr(&self, sys_id: usize) -> usize{
    //     let mut inner = self.inner.borrow_mut();
    //     let total_app_num = get_app_num();
    //     let app_num = inner.app_num;
    //     let current_process = inner.current_process;
    //     let mut next_run = 1;
    //     let mut flag = false;
    //     for _i in 1..=inner.processes.len() {
    //         let j = current_process % total_app_num;
    //         if inner.processes[j].state == ProcessStatus::Ready {
    //             next_run = j;
    //             flag = true;
    //             break;
    //         }
    //     }
        
    //     if sys_id == SYSCALL_EXIT {
    //         inner.app_num -= 1;
    //         inner.processes[current_process - 1].state = ProcessStatus::Exited;
    //     } else {
    //         inner.processes[current_process - 1].state = ProcessStatus::Ready;
    //     }
        
    //     if flag == false {
    //         if app_num == 1 {
    //             return inner.processes[current_process - 1].prepare();
    //         }
    //     }
        
        
    //     if inner.processes[current_process - 1].state == ProcessStatus::Running {
    //         inner.processes[current_process - 1].state = ProcessStatus::Ready;
    //     }
        
    //     inner.current_process = next_run + 1;
    //     inner.processes[next_run].state = ProcessStatus::Running;
    //     inner.processes[next_run].prepare()
    // }
}