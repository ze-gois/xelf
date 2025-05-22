# src/start.s
.global _start
.section .text._start

_start:
    # Setup stack frame and arguments for main
    xor %rbp, %rbp       # Clear frame pointer
    pop %rdi             # argc -> first argument
    mov %rsp, %rsi       # argv -> second argument

    # Calculate envp position: skip over argv array
    lea 8(%rsi,%rdi,8), %rdx  # envp -> third argument

    # Find end of envp
    mov %rdx, %rcx       # Save envp in rcx
1:
    cmp $0, (%rcx)       # Check for NULL terminator
    je 2f                # If NULL found, exit loop
    add $8, %rcx         # Move to next env pointer
    jmp 1b               # Continue loop

2:  # rcx now points to the NULL terminator of envp
    add $8, %rcx         # Move past NULL to start of auxv
    mov %rcx, %r8        # auxv -> fourth argument

    # Align stack to 16 bytes
    and $-16, %rsp

    # Preserve fourth argument in r8
    mov %r8, %rcx        # Move auxv pointer to rcx (4th argument)

    call main            # Call Rust main function

    # Exit
    mov %rax, %rdi      # Use main's return value as exit status
    mov $60, %rax       # exit syscall
    syscall
