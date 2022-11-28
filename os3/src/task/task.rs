/*
 * @Author: firesunny
 * @Date: 2022-11-26 20:15:49
 * @LastEditTime: 2022-11-29 02:04:41
 * @FilePath: /lab1-os3-firesunny0/os3/src/task/task.rs
 * @Description:
 */
use super::get_time_us;
use super::TaskContext;
use crate::config::*;
use alloc::vec;
use alloc::vec::*;

#[derive(Debug, Clone)]
pub struct TaskInfo {
    pub status: TaskStatus,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    //: time
    // real time
    pub time: usize,
    pub sys_time: usize,
    pub usr_time: usize,
    // resume_time
    pub resume_time: usize,
    // total time
    pub start_time: usize,
    pub end_time: usize,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TaskStatus {
    UnInit,  // 未初始化
    Ready,   // 准备运行
    Running, // 正在运行
    Exited,  // 已退出
}

#[derive(Clone)]
pub struct TaskControlBlock {
    // base
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    pub task_info: TaskInfo,
}

impl TaskInfo {
    pub fn new() -> Self {
        TaskInfo {
            status: TaskStatus::UnInit,
            syscall_times: [0; MAX_SYSCALL_NUM],
            time: 0,
            resume_time: 0,
            sys_time: 0,
            usr_time: 0,
            start_time: 0,
            end_time: 0,
        }
    }

    pub fn real_time(&self) -> usize {
        get_time_us() / 1000 - self.start_time
    }
    //
    pub fn cal_time_before_suspend(&mut self) {
        //: calculate run time
        self.time += get_time_us() / 1000 - self.resume_time;
    }
    pub fn cal_time_before_resume(&mut self) {
        self.resume_time = get_time_us() / 1000;
        if self.start_time == 0 {
            self.start_time = self.resume_time;
        }
    }
    pub fn cal_time_before_exited(&mut self) {
        //: calculate time
        // total time
        let cur_time = get_time_us() / 1000;
        self.time += cur_time - self.resume_time;
        self.end_time = cur_time;
    }
}
