/* AArch64 Boot Assembly Entry Point */
.section ".text.boot"
.global _start

_start:
    /* Ensure we are in EL2 (hypervisor mode) or EL1 (kernel mode) */
    mrs x0, CurrentEL
    and x0, x0, #0xC
    cmp x0, #8          /* Check if EL2 */
    b.eq .setup_sp
    
    /* If not EL2, assume EL1 is fine for our purposes */
    /* In a real OS, you might want to handle EL3 or drop privileges */
    
.setup_sp:
    /* Set up stack pointer to a safe location */
    /* The bootloader should have set up memory, so we use a high address */
    ldr x0, =_end
    mov sp, x0
    
    /* Clear BSS section */
    ldr x0, =__bss_start
    ldr x1, =__bss_end
    cmp x0, x1
    b.eq .bss_done
    
.clear_bss:
    str xzr, [x0]
    add x0, x0, #8
    cmp x0, x1
    b.lt .clear_bss
    
.bss_done:
    /* Jump to Rust main function */
    /* Pass DTB pointer in x0 (as per boot protocol) */
    bl main
    
    /* Hang if main returns (it shouldn't) */
.hang:
    wfi
    b .hang
