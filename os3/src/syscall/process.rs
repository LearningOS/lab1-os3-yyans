//! Process management syscalls

use alloc::task;

use crate::config::{MAX_APP_NUM, MAX_SYSCALL_NUM};
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus, get_task_info};
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

// #[derive(Clone, Copy)]
pub struct TaskInfo {
    // pub status: TaskStatus,
    // pub syscall_times: [u32; MAX_SYSCALL_NUM],
    // pub time: usize,
    status: TaskStatus,
    syscall_times: [u32; MAX_SYSCALL_NUM],
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let current_time = get_time_us() / 1000;
    let task_info = get_task_info();
    unsafe {
        *ti = TaskInfo {
            status: task_info.task_status,
            syscall_times: [0; MAX_SYSCALL_NUM],
            time: current_time - task_info.start_time,
        }
    }
    -1
}
