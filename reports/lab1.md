## lab1
2021010724 曾宪伟

### 实现的功能

- #### sys_task_info
    直接将控制块信息写到参数指向的内存地址。发生syscall时该任务块一定处于 `running` 状态。每个`block`维护一个 `bool` 变量代表是否是第一次进行syscall，运行一个新任务时，若是第一次系统调用，记录下当前时刻。查询时长时用当前时刻减去初次调用时刻。`block`新增一个结构体数组维护每个调用号对应的次数，在`syscall`函数中更新。

### 简答作业
- #### 1.
    `[rustsbi] RustSBI version 0.3.0-alpha.2, adapting to RISC-V SBI v1.0.0`
    三个程序的报错分别为
    ch2b_bad_address.rs
    `[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003ac, kernel killed it.`
    ch2b_bad_instructions.rs
    `[kernel] IllegalInstruction in application, kernel killed it.`
    ch2b_bad_register.rs
    `[kernel] IllegalInstruction in application, kernel killed it.`