/*
 * @Author: firesunny
 * @Date: 2022-11-26 19:55:56
 * @LastEditTime: 2022-11-29 01:34:05
 * @FilePath: /lab1-os3-firesunny0/os3/src/task/mod.rs
 * @Description:
 */

mod context;
mod switch;
mod task;

use crate::config::*;
use crate::loader::*;
use crate::sync::UPSafeCell;
use crate::timer::get_time_us;
use alloc::vec;
use alloc::vec::*;
use context::*;
use lazy_static::lazy_static;
use switch::*;
pub use task::*;

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        // let success_app = 0;
        // let failed_app = 0;
        info!("num_app : {}", num_app);
        let mut tasks = vec![
            TaskControlBlock {
                task_cx: TaskContext::zero_init(),
                task_status: TaskStatus::UnInit,
                task_info: TaskInfo::new(),
            };
            MAX_APP_NUM
        ];
        for i in 0..num_app {
            tasks[i].task_cx = TaskContext::goto_restore(init_app_cx(i));
            tasks[i].task_status = TaskStatus::Ready;
        }
        TaskManager {
            num_app,
            // success_app,
            // failed_app,
            inner: unsafe {
                UPSafeCell::new(TaskManagerInner {
                    tasks,
                    current_task: 0,
                })
            },
        }
    };
}

pub fn run_first_task() {
    debug!("func : run_first_task");
    TASK_MANAGER.run_first_task();
}

pub fn exit_current_and_run_next() {
    mark_current_exited();
    run_next_task();
}

pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

pub fn syscall_times_count(syscall_id: usize) {
    TASK_MANAGER.syscall_times_count(syscall_id);
}

pub fn get_current_task_info(task_info: &mut TaskInfo) {
    TASK_MANAGER.get_current_task_info(task_info);
}

fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
    // success_app: usize,
    // failed_app: usize,
}

struct TaskManagerInner {
    tasks: Vec<TaskControlBlock>,
    current_task: usize,
}

impl TaskManager {
    fn mark_current_suspended(&self) {
        debug!("mark current suspended ...");
        let mut inner = self.inner.exclusive_access();
        let current_index = inner.current_task;
        debug!("mark current suspended : {}", current_index);
        let cur_task = &mut inner.tasks[current_index];

        cur_task.task_info.cal_time_before_suspend();
        cur_task.task_status = TaskStatus::Ready;

        debug!("current has suspended");
    }

    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current_index = inner.current_task;
        debug!("mark current exited : {}", current_index);
        let cur_task = &mut inner.tasks[current_index];

        cur_task.task_info.cal_time_before_exited();

        //: mark exited
        cur_task.task_status = TaskStatus::Exited;
        debug!("current has exited");
    }

    fn run_first_task(&self) -> ! {
        info!("start first task!");
        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        task0.task_status = TaskStatus::Running;
        let next_task_cx_ptr = &task0.task_cx as *const TaskContext;
        let mut _unused = TaskContext::zero_init();

        task0.task_info.cal_time_before_resume();

        drop(inner);
        info!("exec __switch for first task");
        // before this, we should drop local variables that must be dropped manually
        unsafe {
            __switch(&mut _unused as *mut TaskContext, next_task_cx_ptr);
        }
        panic!("unreachable in run_first_task!");
    }

    fn run_next_task(&self) {
        debug!("run next task ...");
        if let Some(next) = self.find_next_task() {
            debug!("next task id is :{}", next);
            let mut inner = self.inner.exclusive_access();
            let current_index = inner.current_task;
            inner.current_task = next;

            let cur_task = &mut inner.tasks[current_index];
            let current_task_cx_ptr = &mut cur_task.task_cx as *mut TaskContext;

            let next_task = &mut inner.tasks[next];
            next_task.task_status = TaskStatus::Running;
            let next_task_cx_ptr = &mut next_task.task_cx as *const TaskContext;

            next_task.task_info.cal_time_before_resume();

            drop(inner);

            // before this, we should drop local variables that must be dropped manually
            unsafe {
                __switch(current_task_cx_ptr, next_task_cx_ptr);
            }
            // go back to user mode
        } else {
            panic!("All applications completed!");
        }
    }

    fn find_next_task(&self) -> Option<usize> {
        debug!("find next task");
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| inner.tasks[*id].task_status == TaskStatus::Ready)
    }

    fn syscall_times_count(&self, syscall_id: usize) {
        debug!("syscall count : id is {} ", syscall_id);
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_info.syscall_times[syscall_id] += 1;
        drop(inner);
    }

    fn get_current_task_info(&self, task_info: &mut TaskInfo) {
        let mut inner = self.inner.exclusive_access();
        let current_index = inner.current_task;
        let cur_task = &mut inner.tasks[current_index];
        cur_task.task_info.status = cur_task.task_status;
        task_info.status = cur_task.task_status;
        task_info.time = cur_task.task_info.real_time();
        (0..MAX_SYSCALL_NUM)
            .for_each(|i| task_info.syscall_times[i] = cur_task.task_info.syscall_times[i]);
    }

    // fn before_mark_suspend(&self) {}

    // fn before_mark_exited(&self) {}
}

// use crate::task::exit_current_and_run_next;
// use crate::task::suspend_current_and_run_next;
