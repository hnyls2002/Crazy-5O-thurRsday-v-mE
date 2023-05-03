## Implementation

#### Address Space

Some Principles
- RAII : Resource Acquisition Is Initialization
- Memory set is a new level of abstraction, which I thought is actually the same as address space.

Implementation Details
- `FrameTacker` manage the source of physical memory. The frames will be free when `FrameTracker` is dropped.
- page table's frame is managed by `PageTable.pt_frames` field.
- physical memory source is managed by `MapArea.mem_frames` field.
  - **`Identical` mapping strategy sources don't need to be managed**
  - They don't acquire physical memory from frame allocator.
  - *available frames section* need to be used by others...

Some structs and methods

- [x] `Page`,`Frame`
- [x] `VirtAddr`,`PhysAddr`
- [x] `PTE` struct
 - with `ppn` field and `flags` field
- [x] fram allocator : stack based
 - `alloc` and `dealloc` methods
 - global frame allocator instance

- [x] RAII based `frameTracker`
 - `drop` method is needed
 - *each page table is a frame*, which is RAII by `frameTracker`
 - *frame (original meaning) is just a physical page*, which is RAII by `frameTracker`

- [x] SV39 page table
 - All *page table* managed by `frameTracker`  
 - `map` and `unmap` method for page table
 - PTE/phy page/phy memory visiting methods
 - phy page visiting method *from different address space*


- [x] `MapArea`
 - *Sequential Virtual Address* in a address space
 - All physical resources are managed by `frameTracker`
 - Different mapping strategies
   - `Identical` for lower half of kernel space
   - `Linear` for `trampline` (*shared*)
   - `Framed` for *user space* and *user stack*
 - `permission` for all frames in this area


- [x] `MemorySet` : *address space*
 - `Vec<MapArea>` for all physical memory resources
 - `PageTable` for a tree of page table

- [x] Figure out *kernel address space layout* and *user address space layout*
- [x] `kernel_space_init()`
- [ ] `new_user_space()` for a single application

#### Tasks(User) Related

**Build the user's application**

- [ ] For each application, build it into an `elf` file
- [ ] Place the compiled `elf` file into kernel's memory (`.data` section). ~~As each app's base address are set when *app being compiled*, so just place them into kernel's memory will case the *offset error* problem.~~
- [ ] ~~Move each app's `elf` file (`img` file) into the correct memory address.~~

Then virtual memory on...

- [ ] A common `linker.ld` for all apps with base address set to `0x10000`.(align settings for R-W-X)
- [ ] A `elf` app analyzer to initialize a new user space
 - User's kernel stack
 - User's user stack
 - User's `TrapContext`
 - User's `Trampoline`
- [ ] For each section in `elf` file, map the virtual address to physical address (also create new frames and **copy data** into it)

**TaskContext**
Some context for switch back to an app's kernel stack.

- [ ] fields including
 - `sp` for kernel stack
 - `ra` for `__ctx_restore`
 - callee saved registers : cause `__switch` is a `naked` function
- [ ] `switch` function


**TrapContext**

- [ ] Context fields : `sscratch` here is to store the `TrapContext` pointer.
 - For trap into the right kernel space : `kernel_satp`, `kernel_sp`, `trap_handler`
 - For back to user space : all registers (`sp` is `x2`), `sstatus`, `sepc`
 - we can know user's `satp` in kernel space (memory_set in kernel space...)

- [ ] `__cxt_save` : save context in user space
- [ ] `__cxt_restore` : jump to user space at first then restore context
- [ ] build the `trampline` page
- [ ] `trap_handler` : dispatch traps
 - jump to it from `__cxt_save`
 - it jumps to `__cxt_restore` after handling the trap

**syscall**

- [ ] `yield`
- [ ] `exit`
- [ ] `write`

**Taks manager, may become process in later chapter 5**

- [ ] `TaskInfo` : a struct to record a task's information.
- [ ] `TaskManager` and its methods.

**Trap related topic is a little bit complicated compared to no-address-space case**

A set of procedures for handling a system call

- [ ] `TrapContex` : save U-mode context
- [ ] `TaskContex` : save S-mode context for switching
- [ ] `__switch`

**Interrupt is a hard topic without support of `rustsbi_qemu`**

- [ ] `__mtimer_handler` : no `C` code, as the xv6 tutorial said.
  So in our project, it is : no `rust` code.
- [ ] timer interrupt delegated to S-mode
 - `__mtimer_interrupt` is still needed, which first handle the interrupt, then delegate it to S-mode as a *software timer interrupt*
- [ ] interrupt happening when in S-mode
  - rCore just panic before chapter 9
  - xv6 write a `kerneltrap` function to handle it