	.text
	.align	4
	.globl	_trampo
	.type	_trampo,@function
_trampoline:
	mov	%rsi,	%rsp
	jmp	*%rdi
	/* Should not reach. */
	hlt
