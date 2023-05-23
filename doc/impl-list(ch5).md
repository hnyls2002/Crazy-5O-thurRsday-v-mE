#### Input Support
- [x] UART input : single char, one by one.
If no char incoming, wait for it or `yield` to other tasks.

#### ELF Loader Support
- [x] `build.rs` : embed name of each app into the kernel
- [x] `elf-loader` : load elf file by name

#### Process Structures

- [x] PID allocator : RAII
- [x] Kernel stack allocator : RAII
- [x] `TaskStruct`
- pid
- kernel stack resource
- task status
- trap context
- task context
- user space
- parent of this process    
- children of this process

`TaskStruct` stored in heap, we can access it by `Arc` or `Weak`.
- `Arc<TaskStruct>`.
- `TaskStruct {inner : Mutex<RefCell<TaskStructInner>>}` or `TaskStruct {inner : UPSafeCell<TaskStructInner>}}`
- `Mutex` or `UPSafeCell` : provides `Sync` trait for `Arc`
- `RefCell` : provides interior mutability

***

- [x] `TaskManager` : All tasks (ready tasks).
- [x] `Processor` : only running tasks, for later multi-core support.

- current task in `Processor`
- `idle_task_ctx` : idle task context
- schedule functions


`task A -> idle -> another task B` Now we have a idle control flow, which is not running on any task's kernel stack. It's actually running on this core's booting stack.

***

#### Resources Management

- [x] Resources release support.
- Frames for memory in `MemorySet` : `map_areas` will drop.
- `PageTable` : when dropping `TaskStruct` after `sys_waitpid()`.
- `KernelStack` : RAII, dropped by compiler also after `sys_waitpid()`.
- `trap_contex` : in memory frames.
- `task_contex` : in kernel space.

#### User Program

- [x] `initproc` : init process
- [x] `user_shell` : user shell 

#### Process System Call

- [x] `sys_fork()` : copy exactly the same task.
- Generate a ready process, return to `trap_return` 
  - Forked process doest't get return value of `sys_fork()`, just set it `a0 = 0`
  - Parent process set `a0 = sys_fork()`
- `TaskContext` : set `ra` to `trap_return`
- `TrapContext` : the same as parent process, **except for `kernel_sp`**
- [x] `sys_exec()` : load from elf file
- [x] `sys_waitpid(pid, *exit_code)` : wait for child process, and get exit code. **release the space of task struct**
- `sys_waitpid(-1)` : wait for any child process
- `sys_waitpid()` return `-1` : no child process (of this pid)
- `sys_waitpid()` return `-2` : child process is running
- `sys_waitpit(id) -> id` : child process `id` exit

#### User API : wrapper of system call

- [x] `wait` : `sys_waitpid(-1, *exit_code)`
- [x] `waitpid` : `sys_waitpid(id, *exit_code)`

#### Test Cases

- [x] sleep
- [x] shell
- [x] read_test
- [x] power_7
- [x] power_5
- [x] power_3
- [x] store_fault
- [x] load_fault
- [x] hello
- [x] sleep_simple
- [x] initproc
- [x] yield
- [x] stack_overflow
- [x] forkexec
- [x] forktree
- [x] forktest_simple
- [x] fantastic_text
- [x] matrix
- [x] exit
- [x] usertests-simple
- [x] forktest
- [x] usertests
- [x] forktest2