use core::mem::zeroed;
use riscv::register::sstatus::{self, Sstatus, SPP::*};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Context {
    pub x: [usize; 32],
    pub sstatus: Sstatus,
    pub sepc: usize,
}

impl Default for Context {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

#[allow(unused)]
impl Context {
    pub fn sp(&self) -> usize {
        self.x[2]
    }

    pub fn set_sp(&mut self, value: usize) -> &mut Self {
        self.x[2] = value;
        self
    }

    pub fn ra(&self) -> usize {
        self.x[1]
    }

    pub fn set_ra(&mut self, value: usize) -> &mut Self {
        self.x[1] = value;
        self
    }

    pub fn new(stack_top: usize, entry_point: usize, is_user: bool) -> Self {
        let mut context = Self::default();

        context.set_sp(stack_top);

        context.sepc = entry_point;

        context.sstatus = sstatus::read();

        if is_user {
            context.sstatus.set_spp(User);
        } else {
            context.sstatus.set_spp(Supervisor);
        }

        context
    }
}