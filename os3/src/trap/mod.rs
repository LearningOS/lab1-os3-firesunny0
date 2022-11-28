mod context;

use crate::task::*;
use crate::{syscall::syscall, timer::set_next_trigger};
core::arch::global_asm!(include_str!("trap.S"));

use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Interrupt, Trap},
    sie, stval, stvec,
};

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            debug!("Env Call");
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            debug!("Time Cycle");
            set_next_trigger();
            suspend_current_and_run_next();
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            error!("[kernel] PageFault in application, core dumped.");
            exit_current_and_run_next();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            error!("[kernel] IllegalInstruction in application, core dumped.");
            exit_current_and_run_next();
        }
        Trap::Exception(Exception::InstructionFault) => {
            error!("[kernel] InstructionFault in application, core dumped.");
            exit_current_and_run_next();
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}!",
                scause.cause(),
                stval
            );
        }
    }
    cx
}

pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

pub use context::TrapContext;
