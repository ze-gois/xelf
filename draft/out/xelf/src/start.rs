// use crate::main;

#[no_mangle]
#[link_section = ".text._start"]
pub extern "C" fn _start() -> ! {
    unsafe {
        core::arch::asm!(
            "mov rax, 60",  // syscall number for exit
            "xor rdi, rdi", // exit code 0
            "syscall",
            options(noreturn)
        );
    }
}

// #[no_mangle]
// #[link_section = ".text._start"]
// pub extern "C" fn _start() -> ! {
//     unsafe {
//         core::arch::asm!(
//             "mov rdi, 0",   // just pass 0 for argc
//             "xor rsi, rsi", // null for argv
//             "xor rdx, rdx", // null for envp
//             "xor rcx, rcx", // null for auxv
//             "and rsp, -16", // align stack
//             "call {main}",
//             "mov rdi, rax", // exit code
//             "mov rax, 60",  // exit syscall
//             "syscall",
//             main = sym main,
//             options(noreturn)
//         );
//     }
// }

// #[no_mangle]
// #[link_section = ".text._start"]
// pub extern "C" fn _start() -> ! {
//     unsafe {
//         core::arch::asm!(
//             "xor rbp, rbp",
//             "pop rdi",
//             "mov rsi, rsp",
//             "lea rdx, [rsi + 8*rdi + 8]",
//             "2:",
//             "add rdx, 8",
//             "cmp QWORD PTR [rdx], 0",
//             "jne 2b",
//             "add rdx, 8",
//             "mov rcx, rdx",
//             "and rsp, -16",
//             "call {main}",
//             "mov rdi, rax",
//             "mov rax, 60",
//             "syscall",
//             main = sym main,
//             options(noreturn)
//         );
//     }
// }

#[no_mangle]
#[link_section = ".text._start"]
pub extern "C" fn _start2() -> ! {
    unsafe {
        core::arch::asm!(
        // # src/start.s
        // .global _start
        // .section .text._start

        // _start:
        //     # Setup stack frame and arguments for main
        //     xor %rbp, %rbp       # Clear frame pointer
        //     pop %rdi             # argc -> first argument
        //     mov %rsp, %rsi       # argv -> second argument
        //     lea 8(%rsi,%rdi,8), %rdx  # envp -> third argument

        // 1:  # Find end of envp (marked by NULL)
        //     add $8, %rdx
        //     cmp $0, (%rdx)
        //     jne 1b
        //     add $8, %rdx         # auxv -> fourth argument

        //     and $-16, %rsp       # Align stack to 16 bytes

        //     call main            # Call Rust main function

        //     # Exit
        //     mov %rax, %rdi      # Use main's return value as exit status
        //     mov $60, %rax       # exit syscall
        //     syscall
                )
    }
}
