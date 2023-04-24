# initial the jump between m-mode and s-mode
    .section .text.entry
    .global _start
_start:
    la sp, boot_stack_top
    call machine_start

# set the boot stack
    .section .bss.stack
    .global boot_stack_bottom
boot_stack_bottom:
    .space 4096 * 16 # 16pages
    .global boot_stack_top
boot_stack_top: