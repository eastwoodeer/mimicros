.section .text._start

    .global _start
_start:
    mrs x19, mpidr_el1
    and x19, x19, {CONST_CORE_ID_MASK}
    ldr x20, BOOT_CORE_ID
    cmp x19, x20
    b.ne secondary_loop

    adrp x19, __boot_stack_end
    mov sp, x19

	bl {switch_to_el1}
	bl {init_boot_page_table}
	bl {init_mmu}
	bl {enable_fp}

wait_for_zero_bss:
    adr x19, __bss_start
    adr x20, __bss_end
    cmp x19, x20
    b.eq prepare_rust
    stp xzr, xzr, [x19], 16
    b wait_for_zero_bss

prepare_rust:
    b _start_rust

primary_entry:
    b primary_entry
    /*bl elx_to_el1*/

secondary_loop:
    wfe
    b secondary_loop
