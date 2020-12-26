use lazy_static::*;
use alloc::sync::Arc;
use spin::Mutex;
use crate::process::*;
use alloc::collections::LinkedList;
use crate::syscall::SYSCALL_EXIT;

pub struct Scheduler {
    pub processes: LinkedList<Arc<Process>>,
    pub current_process: Option<Arc<Process>>,
    pub process_ids: LinkedList<isize>,
}

unsafe impl Sync for Scheduler {}

lazy_static! {
    pub static ref SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());
}

impl Scheduler {
    pub fn new() -> Self {
        Scheduler {
            processes: LinkedList::new(),
            current_process: None,
            process_ids: LinkedList::new(),
        }
    }

    pub fn add_process(&mut self, process: Arc<Process>) {
        self.process_ids.push_back(process.id);
        self.processes.push_back(process);
    }

    pub fn get_next(&mut self) -> Option<Arc<Process>> {
        if let Some(process) = self.processes.pop_front() {
            self.process_ids.pop_front();
            Some(process)
        } else {
            None
        }
    }

    // pub fn remove_process(&mut self, process: &Arc<Process>) {
    //     let mut removed = self.processes.drain_filter(|p| p == process);
    //     assert!(removed.next().is_some() && removed.next().is_none());
    // }

    pub fn current_process(&self) -> Option<Arc<Process>> {
        self.current_process.as_ref().map(|process| process.clone())
    }

    pub fn find_process_id(&self, id: isize) -> bool {
        self.process_ids.contains(&id)
    }

    pub fn get_ptr(&mut self, sys_id: usize) -> usize {
        if let Some(process) = self.get_next() {
            if sys_id != SYSCALL_EXIT && self.current_process.is_none() == false {
                let current_process = self.current_process.take().unwrap();
                self.add_process(current_process);
            }

            if self.current_process.is_none() == false {
                self.current_process.take().unwrap();
            }
            let context_ptr = process.prepare();
            self.current_process = Some(process);
            context_ptr
        } else {
            if sys_id == SYSCALL_EXIT {
                self.current_process = None;
                panic!("[K] all app end");
            } else {
                self.current_process.as_ref().unwrap().prepare()
            }
        }
    }
}