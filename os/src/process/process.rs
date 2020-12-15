use crate::interrupt::Context;

#[derive(Copy, Clone)]
pub struct Process {
    pub context_ptr: usize,
    pub state: processStatus,
}

#[derive(Copy, Clone, PartialEq)]
pub enum processStatus {
    Running,
    Waiting,
    Ready,
    Exited,
}