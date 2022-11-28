/*
 * @Author: firesunny
 * @Date: 2022-11-26 20:28:09
 * @LastEditTime: 2022-11-26 21:17:17
 * @FilePath: /lab1-os3-firesunny0/os3/src/task/context.rs
 * @Description:
 */

#[derive(Copy, Clone)]
#[repr(C)]
pub struct TaskContext {
    ra: usize,
    sp: usize,
    s: [usize; 12],
}

impl TaskContext {
    pub fn goto_restore(kstack_ptr: usize) -> Self {
        extern "C" {
            fn __restore();
        }
        Self {
            ra: __restore as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }

    pub fn zero_init() -> Self {
        TaskContext {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }
}
