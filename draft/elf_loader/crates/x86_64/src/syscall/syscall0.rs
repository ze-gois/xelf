#[inline(always)]
pub unsafe fn syscall0(n: usize) -> usize {
    let ret: usize;
    core::arch::asm!(
        "syscall",
        in("rax") n,
        out("rcx") _,
        out("r11") _,
        lateout("rax") ret,
    );
    ret
}
