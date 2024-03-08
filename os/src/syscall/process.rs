//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM, mm::VirtAddr, task::{
        change_program_brk, current_user_token, exit_current_and_run_next, get_syscall_times, get_time_segment, mapped, mmap_alloc, suspend_current_and_run_next, unmap_area, TaskStatus
    }, 
    timer::get_time_us,
};
use crate::mm::PageTable;
use crate::mm::PhysAddr;
use crate::mm::VPNRange;
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    let time_value = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    
    let vaddr = VirtAddr::from(_ts as usize);
    let page_table = PageTable::from_token(current_user_token());
    let ppn = page_table.translate(vaddr.floor()).unwrap().ppn();
    let physical_addr = vaddr.page_offset() + PhysAddr::from(ppn).0;
    let time_val_to_write = physical_addr as *mut TimeVal;
    unsafe {
        *time_val_to_write = time_value;
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    //trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    let vaddr = VirtAddr::from(_ti as usize);
    let page_table = PageTable::from_token(current_user_token());
    let ppn = page_table.translate(vaddr.floor()).unwrap().ppn();
    let physical_addr = vaddr.page_offset() + PhysAddr::from(ppn).0;
    let task_info_to_write = physical_addr as *mut TaskInfo;
    let info = TaskInfo {
        status: TaskStatus::Running,
        syscall_times: get_syscall_times(),
        time: get_time_segment(),
    };
    unsafe {
        *task_info_to_write = info;
    }
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    let start_va = VirtAddr::from(_start);
    let end_va = VirtAddr::from(_start + _len);
    let start_vpn = start_va.floor();
    let end_vpn = end_va.ceil();
    if (_port & !0x7 !=0) || (_port & 0x7 == 0) || 
    (start_va.aligned() == false) {
        return -1;
    }
    let vpn_range = VPNRange::new(start_vpn, end_vpn);
    for vpn in vpn_range {
        if mapped(vpn) {
            return -1;
        }
    }
    // let a: VirtAddr = start_vpn.into();
    // let b: VirtAddr = end_vpn.into();
    // assert!(a.0 == _start);
    // assert!(b.0 == _start + _len);
    // mmap_alloc(start_va, end_va, _port);
    mmap_alloc(start_vpn.into(), end_vpn.into(), _port);
    0
        
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    if VirtAddr::from(_start).aligned() == false {
        return  -1;
    }
    unmap_area(_start, _len)
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
