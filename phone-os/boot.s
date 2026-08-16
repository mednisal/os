.section .text.boot
.global _start

_start:
    mrs     x0, mpidr_el1
    and     x0, x0, #0xFF
    cbnz    x0, hang

    ldr     x1, =_stack_top
    mov     sp, x1

    ldr     x1, =main
    blr     x1

hang:
    wfe
    b       hang

.section .bss
.align 12
.global _stack_bottom
_stack_bottom:
    .space 4096

.global _stack_top
_stack_top:
