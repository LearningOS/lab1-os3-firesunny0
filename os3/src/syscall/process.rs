use crate::task::exit_current_and_run_next;
use crate::task::suspend_current_and_run_next;
use crate::timer::*;

pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

pub fn sys_get_time(time: &mut TimeVal, tz: usize) -> isize {
    time.usec = get_time_us();
    time.sec = time.usec / 1_000_000;
    0
}
