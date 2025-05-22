#[inline(always)]
pub unsafe fn syscall0(n: usize) -> isize {
    let ret: isize;

    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") n as isize => ret,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
    }

    ret
}
