#[inline(always)]
pub fn syscall1(n: usize, a1: usize) -> usize {
    let ret: usize;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") n,
            in("rdi") a1,
            out("rcx") _,
            out("r11") _,
            lateout("rax") ret,
        );
    }
    ret
}
