## Issues with RISC-V QEMU

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