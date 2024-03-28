OUTPUT_ARCH(aarch64)

ENTRY(_start)

BASE_ADDRESS = 0xFFFF000040000000;

/* Flags:
 *     4 == R
 *     5 == RX
 *     6 == RW
 *
 * Segments are marked PT_LOAD below so that the ELF file provides virtual and physical addresses.
 * It doesn't mean all of them need actually be loaded.
 */
PHDRS {
    segment_boot_core_stack PT_LOAD FLAGS(6);
    segment_code            PT_LOAD FLAGS(5);
    segment_data            PT_LOAD FLAGS(6);
}

SECTIONS {
    . = BASE_ADDRESS;

    .boot_core_stack (NOLOAD) : {
        . += 0x80000;

        __boot_stack_end = .;
    } :segment_boot_core_stack

    .text : {
        KEEP(*(.text._start))
        *(.text.boot)
        *(.text*)
        /* ${BOOT_OBJECTS} */
    } :segment_code

    . = ALIGN(4K);

    boot_end = ABSOLUTE(.);

    .rodata : ALIGN(8) {
        *(.rodata*)
    } :segment_code

    .data : ALIGN(4K) {
        *(.data.boot_pgtable)
        . = ALIGN(4K);
        *(.data*)
    }

    . = ALIGN(4K);
	_per_cpu_start = .;
	.per_cpu : AT(_per_cpu_start) {
	    _per_cpu_load_start = .;
		*(.per_cpu .per_cpu.*)
		_per_cpu_load_end = .;
		. = ALIGN(64);
		_per_cpu_size_align_end = .;

        . = _per_cpu_load_start + _per_cpu_size_align_end * 4; /* 4 cores */
	}
	. = _per_cpu_start + SIZEOF(.per_cpu);
	_per_cpu_end = .;


    .bss (NOLOAD) : ALIGN(16)
    {
        __bss_start = .;
        *(.bss*);
        . = ALIGN(16);
        *(.bss.stack);
        . = ALIGN(8);
        __bss_end = .;
    } :segment_data
}
