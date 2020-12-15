use super::*;

#[repr(align(16))]
#[derive(Copy, Clone)]
pub struct UserStack{
    stack: [u8; USER_STACK_SIZE],
}

pub static mut USER_STACK: [UserStack; 3] = [
    UserStack{ stack: [0; USER_STACK_SIZE] };
    3
];

impl UserStack {
    pub fn get_sp(&self) -> usize {
        self.stack.as_ptr() as usize + USER_STACK_SIZE
    }
}