use super::context::Context;
use crate::syscall::syscall;
use riscv::register::{
    sstatus::{Sstatus, self, SPP},
    scause::{Exception, Interrupt, Scause, Trap},
    stvec,
};

global_asm!(include_str!("./interrupt.asm"));

pub fn init() {
    unsafe {
        extern "C" {
            fn __interrupt();
        }
        //设置中断处理函数位置以及中断模式
        stvec::write(__interrupt as usize, stvec::TrapMode::Direct);
    }
}

#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) -> &mut Context{
    // println!("[S] handle interrupt");
    // println!("spp: {:?}", context.sstatus.spp());
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => handle_syscall(context),
        Trap::Exception(Exception::Breakpoint) => break_point(context),
        _ => fault(context, scause, stval),
    }
    context
}

fn handle_syscall(context: &mut Context) {
    //指向系统调用回来的下一条指令
    context.sepc += 4;
    //存储系统调用返回值
    context.x[10] = syscall(context.x[17], context.x[10], context.x[11], context.x[12]);
}

fn break_point(context: &mut Context) {
    println!("Breakpoint at 0x{:x}", context.sepc);
    context.sepc += 2;

}

/// 出现未能解决的异常
fn fault(context: &mut Context, scause: Scause, stval: usize) {
    panic!(
        "Unresolved interrupt: {:?}\n{:x?}\nstval: {:x}",
        scause.cause(),
        context,
        stval
    );
}