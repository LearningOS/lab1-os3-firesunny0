const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GETTIMEOFDAY: usize = 169;
const SYSCALL_TASK_INFO: usize = 410;

mod fs;
mod process;

use crate::task::syscall_times_count;
use crate::task::TaskInfo;
use crate::timer::TimeVal;
use fs::sys_write;
use process::*;

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    syscall_times_count(syscall_id);
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_TASK_INFO => unsafe { sys_task_info(&mut (*(args[0] as *mut TaskInfo))) },
        SYSCALL_GETTIMEOFDAY => unsafe {
            sys_get_time(&mut (*(args[0] as *mut TimeVal)), args[1] as usize)
        },
        _ => panic!("Unsupported syscall_id: {}", syscall_id),
    }
}
