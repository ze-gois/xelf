	.text
	.align	4
	.globl	_start
	.hidden	_start
	.section .text._start
	.type	_start,@function
_start:
	mov	%rsp,	%rdi
	call	main
	hlt
