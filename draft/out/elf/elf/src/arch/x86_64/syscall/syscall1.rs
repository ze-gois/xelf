#[inline(always)]
pub unsafe fn syscall1(n: usize, a1: usize) -> isize {
    let ret: isize;

    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") n as isize => ret,
            in("rdi") a1,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
    }

    ret
}
