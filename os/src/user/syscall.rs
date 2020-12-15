#![allow(unused)]
fn syscall(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which)
            : "memory"
            : "volatile");
    }
    ret
}

const SYS_WRITE: usize = 1;
const SYS_EXIT: usize = 2;
const SYS_YIELD: usize = 3;

pub fn sys_write(fd: usize, buf: &[u8]) -> usize{
    syscall(SYS_WRITE, fd, buf.as_ptr() as usize, buf.len())
}

pub fn sys_exit(c: usize) -> usize{
    syscall(SYS_EXIT, c, 0 ,0)
}

pub fn sys_yield() -> usize{
    syscall(SYS_YIELD, 0, 0 ,0)
}