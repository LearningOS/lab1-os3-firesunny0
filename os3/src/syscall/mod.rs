const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GETTIMEOFDAY: usize = 169;

mod fs;
mod process;

use crate::task::exit_current_and_run_next;
use crate::timer::*;
use fs::*;
use process::*;

pub fn syscall(syscall_id: usize, args: [usize; 3]) -> isize {
    match syscall_id {
        SYSCALL_WRITE => sys_write(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit(args[0] as i32),
        SYSCALL_YIELD => sys_yield(),
        SYSCALL_GETTIMEOFDAY => {
            // let time;
            let time;
            unsafe {
                time = &mut *(args[0] as *mut TimeVal);
            }
            sys_get_time(time, args[1] as usize)
        }
        // _ => panic!("Unsupported syscall_id: {}", syscall_id),
        _ => {
            warn!("Unsupported syscall_id: {}", syscall_id);
            exit_current_and_run_next();
            0
        }
    }
}
