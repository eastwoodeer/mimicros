ENTRY(_start)

BASE_ADDRESS = 0x80000;

SECTIONS {
    . = BASE_ADDRESS;

    .text.boot : {
	    KEEP(*(.text._start))
        *(.text.boot)
        /* ${BOOT_OBJECTS} */
    }
	. = ALIGN(4K);

	boot_end = ABSOLUTE(.);
}
