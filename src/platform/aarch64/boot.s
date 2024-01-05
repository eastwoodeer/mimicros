.section .text._start

    .global _start
_start:
    mrs x19, mpidr_el1
    and x19, x19, {CONST_CORE_ID_MASK}
    ldr x20, BOOT_CORE_ID
    // x19 is callee saved register, after rust functions x19 will not changed.
    // here x19 is used for saving the cpuid
    cmp x19, x20
    b.ne secondary_loop

    adrp x21, __boot_stack_end
    mov sp, x21

    bl {switch_to_el1}
    bl {init_boot_page_table}
    bl {init_mmu}
    bl {enable_fp}

wait_for_zero_bss:
    adr x21, __bss_start
    adr x22, __bss_end
    cmp x21, x22
    b.eq prepare_rust
    stp xzr, xzr, [x21], 16
    b wait_for_zero_bss

prepare_rust:
    mov x0, x19
    ldr x8, ={rust_start_main}
    blr x8
    b .

primary_entry:
    b primary_entry
    /*bl elx_to_el1*/

secondary_loop:
    wfe
    b secondary_loop
