## Issues

#### M-mode mret to S-mode
- Set `mstatus.mpp` to `S-mode`, not `U-mode` or `M-mode`
- Set `mstatus.mepc` to supervisor main function

#### Interrupts and Exceptions Delegation
- `mideleg` settings
- `medeleg` settings

#### Interrupts Enable
- ~~`mstatus.mie` settings~~
- `sstatus.sie` settings

#### Timer Interrupts Delegation
- TBD   

#### Trap Vector Settings
- `Direct` or `Vectored` mode, which is placed at `mtvec` or `stvec`'s **last 2 bits**.
- So **four bytes** aligned is needed for `mtvec` or `stvec`.

#### `repr(C)` and `repr(aligned(4096))`

- `repr(C)` - Align the fields as the same as the `C` compiler.
```rust
// Two rules:
// 1. The size of a struct is the multiple of the largest alignment of all fields.
// 2. For each field, the offset of the field is a multiple of its alignment.

#[repr(C)]
pub struct Test1 {
    pub ch : u8, // 1 bytes + 3 bytes padding
    pub a : i32, // 4 bytes + 0 bytes padding
    pub b : i16, // 2 bytes + 2 bytes padding
}

#[repr(C)]
pub struct Test2 {
    pub d : i64, // 8 bytes
    pub f : f64, // 8 bytes
    pub g : i32, // 4 bytes
    pub h : i16, // 4 bytes
}
```
- `repr(align(4096))` - Align the struct to 4096 bytes.

#### User apps' build process
1. `make build` generate all `.elf` and `.bin` files
2. `os/build.rs` is a build script which automatically runs before the `cargo build` command. In this script, we generate the `link_app.S` to `.incbin` all the user apps' code.
3. When running kernel, for each app, load the `elf` into memory (yes, it does copy into the newly acquired frames).

`link_app.S` : we use `.quad` to store the address of each app's elf/bin file. 

Why we can't directly use the symbols but store the symbols into `.quad` section ? 
- Ans: as the kernel img strips all symbols and headrs, so symbols can only be recognized at compile time. However, we don't know the number of apps at compile time...

#### `ELF` section's memory size vs file size
- Memory size can be larger than file size
- Some space in memory would not appear in the file, such as `.bss` section.

#### Jump Address When Handling Traps
- The location of `__save_ctx` and `__restore_ctx` in `trampoline` is wrong by `extern "C"` export. As we can only see the physical address, but `trampoline` will be mapped to the highest virtual page.
- From `__save_ctx` to `trap_handler` : just `jr` to the symbol.
- From `trap_handler` to `__restore_ctx` : calculate `__restore_ctx`'s virtual address by offset (trampoline page + offset).
- The jump address :
  - To `trap_handler` : in `TrapContext`
  - To `__restore_ctx` : by function `trap_return()`
  - `TaskContext.ra` is for *return address* in kernel function.

#### Apps' Kernel Stack

Each time traping into kernel, the kernel stack of this trapped app is empty...