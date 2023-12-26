.section .text._start

_start:
    // x0 saved for boot cpu id.
    mrs x0, mpidr_el1
    and x0, x0, {CONST_CORE_ID_MASK}
	ldr x1, BOOT_CORE_ID
	cmp x0, x1
	b.ne secondary_loop

wait_for_zero_bss:
	adr x0, __bss_start
	adr x1, __bss_end
	cmp x0, x1
	b.eq prepare_rust
	stp xzr, xzr, [x0], 16
	b wait_for_zero_bss


prepare_rust:
	adr x0, boot_stack
	mov sp, x0
	b _start_rust

primary_entry:
	b primary_entry
    /*bl elx_to_el1*/

secondary_loop:
	wfe
	b secondary_loop
