.macro INVALID_EXCP, kind, source
	.align 7
	mov x0, sp
	mov x1, \kind
	mov x2, \source
	bl invalid_exception
	b exception_exit
.endm

	.section .text
	.align 11
	.global exception_vector_base
exception_vector_base:
	// current EL, SP_EL0
	INVALID_EXCP 0 0
	INVALID_EXCP 1 0
	INVALID_EXCP 2 0
	INVALID_EXCP 3 0

	// current EL, SP_ELx
	INVALID_EXCP 0 1
	INVALID_EXCP 1 1
	INVALID_EXCP 2 1
	INVALID_EXCP 3 1

	// current EL, aarch64
	INVALID_EXCP 0 2
	INVALID_EXCP 1 2
	INVALID_EXCP 2 2
	INVALID_EXCP 3 2

	// current EL, aarch32
	INVALID_EXCP 0 3
	INVALID_EXCP 1 3
	INVALID_EXCP 2 3
	INVALID_EXCP 3 3
exception_exit:
	eret
