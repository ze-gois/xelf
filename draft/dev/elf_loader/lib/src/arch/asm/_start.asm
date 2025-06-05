	.text
	.align	4
	.globl	_start
	.hidden	_start
	.type	_start,@function
_start:
	mov	%rsp,	%rdi
	mov	%rdx,	%rsi
	call	_entry
	hlt
