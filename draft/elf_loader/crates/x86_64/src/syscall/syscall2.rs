#[inline(always)]
pub unsafe fn syscall2(n: usize, a1: usize, a2: usize) -> usize {
    let ret: usize;
    core::arch::asm!(
        "syscall",
        in("rax") n,
        in("rdi") a1,
        in("rsi") a2,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
    );
    ret
}
