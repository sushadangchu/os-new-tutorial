use super::*;

#[repr(align(16))]
#[repr(C)]
pub struct UserStack([u8; USER_STACK_SIZE]);

pub static mut USER_STACK: UserStack = UserStack([0; USER_STACK_SIZE]);

impl UserStack {
    pub fn get_sp(&self) -> usize {
        self.0.as_ptr() as usize + USER_STACK_SIZE
    }
}