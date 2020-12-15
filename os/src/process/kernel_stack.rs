use super::*;
use core::mem::size_of;

#[repr(align(16))]
#[derive(Copy, Clone)]
pub struct KernelStack{
    stack: [u8; KERNEL_STACK_SIZE],
}

pub static mut KERNEL_STACK: [KernelStack; SATACK_NUM] = [
    KernelStack { stack: [0; KERNEL_STACK_SIZE], };
    SATACK_NUM
];

impl KernelStack {
    /// 在栈顶加入 Context 并且返回新的栈顶指针
    pub fn push_context(&mut self, context: Context) -> *mut Context {
        // 栈顶
        let stack_top = &self.stack as *const _ as usize + size_of::<Self>();
        // Context 的位置
        let push_address = (stack_top - size_of::<Context>()) as *mut Context;
        unsafe {
            *push_address = context;
        }
        push_address
    }

    pub fn get_sp(&self) -> usize {
        self.stack.as_ptr() as usize + KERNEL_STACK_SIZE
    }
}