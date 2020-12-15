use lazy_static::*;
use core::cell::RefCell;
use crate::process::*;
use crate::interrupt::*;
use crate::user::*;
use crate::syscall::SYS_EXIT;

const APP_NUM: usize = 4;

pub struct Scheduler {
    pub inner: RefCell<SchedulerInner>,
}

pub struct SchedulerInner {
    pub processes: [Process; APP_NUM],
    pub current_process: usize,
    pub app_num: usize,
}

unsafe impl Sync for Scheduler {}

lazy_static! {
    pub static ref SCHEDULER: Scheduler = {
        let app_addr = [power_3 as usize, power_5 as usize, power_7 as usize, sleep as usize];
        
        let mut processes = [
            Process { context_ptr: 0, state: ProcessStatus::Ready };
            APP_NUM
        ];


        unsafe {
            for i in 0..app_addr.len() {
                let context = Context::new(USER_STACK[i].get_sp(), app_addr[i] as usize, true);
                processes[i].context_ptr = KERNEL_STACK[i].push_context(context) as * const _ as usize;
            }
        }
        Scheduler {
            inner: RefCell::new(SchedulerInner {
                processes,
                current_process: 0,
                app_num: app_addr.len(),
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
        let app_num = inner.app_num;
        let current_process = inner.current_process;
        let mut next_run = 0;
        let mut flag = false;
        for i in 1..=inner.processes.len() {
            let j = (i + current_process) % APP_NUM;
            if inner.processes[j].state == ProcessStatus::Ready {
                next_run = j;
                flag = true;
                break;
            }
        }

        if sys_id == SYS_EXIT {
            inner.app_num -= 1;
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
        inner.processes[next_run].context_ptr
    }
}