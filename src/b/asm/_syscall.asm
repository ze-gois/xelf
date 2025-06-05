# User space ABI:
# 0  rdi,
# 1  rsi,
# 2  rdx,
# 3  rcx,
# 4  r8,
# 5  r9, the rest args on the stack
#
# Kernel spaceABI:
# 0  rax (nsys),
# 1  rdi,
# 2  rsi,
# 3  rdx,
# 4  r10,
# 5  r8,
# 6  r9 (stack pointer)
	.text
	.align	4
	.globl	_syscall
	.type	_syscall,@function
_syscall:
	mov	%rdi, 	%rax
	mov	%rsi, 	%rdi
	mov	%rdx, 	%rsi
	mov	%rcx, 	%rdx
	mov	%r8,  	%r10
	mov	%r9,  	%r8
	mov	8(%rsp),%r9
	syscall
	ret
