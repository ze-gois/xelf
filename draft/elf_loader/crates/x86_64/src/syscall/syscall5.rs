#[inline(always)]
pub unsafe fn syscall5(n: usize, a1: usize, a2: usize, a3: usize, a4: usize, a5: usize) -> usize {
    let ret: usize;
    core::arch::asm!(
        "syscall",
        in("rax") n,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        in("r10") a4,
        in("r8") a5,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
    );
    ret
}
