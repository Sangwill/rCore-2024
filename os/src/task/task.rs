//! Types related to task management

use crate::config::SYSCALL_NUM;

use super::TaskContext;
/// syscall ID and times corresponding
#[derive(Copy, Clone)]
pub struct IDTimesPair {
    /// syscall ID
    pub syscall_id: usize,
    /// corresponding syscall times
    pub syscall_times: u32,
}


/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The time when first called
    pub first_call_time: usize,
    /// Whether first call
    pub first_call: bool,
    /// syscall id and times
    pub id_times_pairs: [IDTimesPair; SYSCALL_NUM],
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
