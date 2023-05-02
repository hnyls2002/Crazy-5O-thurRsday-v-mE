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