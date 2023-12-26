.section .text._start

_start:
.L_looping:
	wfe
	b .L_looping
