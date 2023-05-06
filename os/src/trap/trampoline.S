# use the macro to save and restore the general purpose registers
.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro RESTORE_GP n
    ld x\n, \n*8(sp)
.endm
    .section .text.trampoline
    .globl __save_trap_ctx
    .globl __restore_trap_ctx
    .align 2 # !!!! 4 bytes align
__save_trap_ctx:
    # sp -> trap_ctx
    csrrw sp, sscratch, sp
    # save the general purpose registers
    sd x1, 1*8(sp)
    sd x3, 3*8(sp)
    .set n, 5
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr
    # save sp
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)
    csrr t0, sscratch
    sd t0, 2*8(sp)
    # ------------ all context saved ------------
    ld t0, 34*8(sp) # kernel_satp
    ld t1, 36*8(sp) # trap_handler
    ld sp, 35*8(sp) # set sp to kernel_sp

    sfence.vma
    csrw satp, t0 # into kernel space
    sfence.vma # flush TLB

    jr t1

# restore needs to know :
# a0 : user_satp to back to user space
# a1 : trap_ctx pointer
__restore_trap_ctx:

    sfence.vma
    csrw satp, a0 # back to user space
    sfence.vma # flush TLB

    mv sp, a1 # sp -> trap_ctx
    csrw sscratch, sp # set sscratch to trap_ctx
    ld t0, 32*8(sp) # sstatus
    ld t1, 33*8(sp) # sepc
    csrw sstatus, t0
    csrw sepc, t1

    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        RESTORE_GP %n
        .set n, n+1
    .endr
    ld sp, 2*8(sp) # restore sp
    sret