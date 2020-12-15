#[derive(Copy, Clone)]
pub struct Process {
    pub context_ptr: usize,
    pub state: ProcessStatus,
}

#[derive(Copy, Clone, PartialEq)]
pub enum ProcessStatus {
    Running,
    Ready,
    Exited,
}