.section .text._start

    .global _start
_start:
    // x0 saved for boot cpu id.
    mrs x0, mpidr_el1
    and x0, x0, {CONST_CORE_ID_MASK}
    ldr x1, BOOT_CORE_ID
    cmp x0, x1
    b.ne secondary_loop

    adrp x8, {boot_stack} //__boot_core_stack_end
	add x8, x8, 40960
    mov sp, x8

	bl {switch_to_el1}
	//bl {init_boot_page_table}
	//bl {init_mmu}
	bl {enable_fp}

//wait_for_zero_bss:
//    adr x0, __bss_start
//    adr x1, __bss_end
//    cmp x0, x1
//    b.eq prepare_rust
//    stp xzr, xzr, [x0], 16
//    b wait_for_zero_bss

prepare_rust:
    b _start_rust

primary_entry:
    b primary_entry
    /*bl elx_to_el1*/

secondary_loop:
    wfe
    b secondary_loop
