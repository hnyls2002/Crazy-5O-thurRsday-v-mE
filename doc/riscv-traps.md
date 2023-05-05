
#### Traps : Exception and Interrupt
- Exception : by cpu, **synchronous**
- Interrupt : by other device, **asynchronous**

![image-20230506001335362](./assets/image-20230506001335362.png)

- All traps are handled by *machine mode* by default, whatever the current mode when trap occurs.
- Set `mideleg` and `medeleg` to delegate the traps to **S-mode**

#### Delegation Rules
- High-privilege happened exception can not be delegated to lower privilege mode.
- Some exceptions can not be delegated to supervisor mode, such as *Environment call from M-mode*.
- Machine interrupts can not be delegated to supervisor mode, such as *machine timer interrupt*.
- >Delegated interrupts result in the interrupt being masked at the delegator privilege level. For example, if the supervisor timer interrupt (STI) is delegated to S-mode by setting mideleg[5], STIs will not be taken when executing in M-mode. By contrast, if mideleg[5] is clear, STIs can be taken in any mode and regardless of current mode will transfer control to M-mode. 
- ...

#### How Environment Call Works

- We delegated the `Environment call from U-mode` to `S-mode`, so we can use `ecall` instruction in user mode to trap into supervisor mode.
- In *xv6*, when trapping into kernel, the `stvev` will be set to `kernelvec` to handle the **trap from kernel**, which is **horizontally** handled.
- In *rustsbi-qemu*, Env-call frome supervisor and Env-call from machine are not delegated, so **we can use sbicall**.

#### sstatus and mstatus
No magic here, just convention.
- When `sret` : return to `sstatus.spp` mode and set `pc = sstatus.sepc`.
- When `mret` : return to `mstatus.mpp` mode and set `pc = mstatus.mepc`.

#### sstatus.sie and mstatus.mie
- When current mode is U-mode, `sstatus.sie` is ignored.
- When current mode is S-mode, interrupts can happen in S-mode when `sstatus.sie` is set.
- `mstatus.mie` is similar to `sstatus.sie`.

#### sstatus.spie
- *Supervisor Previous Interrupt Enable*
- When env-call happens, `sstatus.spie` will save `sstatus.sie` and `sstatus.sie` will be cleared temporarily. (This is done by hardware)
- So **nested interrupts** would not happen by default.
- *vx6* enables `sstatus.sie` when in kernel, so **it can handle the timer interrupt even in S-mode**.


#### SIE and MIE registers
- `sie.ssie`, `sie.stie`, `sie.seie` to control 3 different kind of interrupts. 
- `mie.msie`, `mie.mtie`, `mie.meie`, `mie.ssie`, `mie.stie`, `mie.seie` to control 6 different kind of interrupts.

#### SIP and MIP regsiters
- **Interrupt Pending** registers
- There are also `ssip`, `stip`, `seip`, `msip`, `mtip`, `meip`
- We can manually raise a interrupt by setting the *interrupt pending* regsiters.

#### Timer Delegation
- For some reason, we can **only use the Machine Timer Interrupt**.
- MTI can't be delegated to S-mode.
- Set `mtvec` to a function `mtimer` to manually delegate MTI : by setting `sip`.
  - `xv6` generates a *supervisor software interrupt* : `sip.ssip`
  - `rustsbi-qemu` generates a *supervisor timer interrupt* : `sip.stip`

So when M-mode handles MTI done, the new *supervisor interrupt* would work.

#### Physical Memoery Protection

According to *xv6*, set `pmpaddr0` and `pmpcfg0`.

#### Trap Vector Align
- `Direct` or `Vectored` mode, which is placed at `mtvec` or `stvec`'s **last 2 bits**.
- So **four bytes** aligned is needed for `mtvec` or `stvec`.
