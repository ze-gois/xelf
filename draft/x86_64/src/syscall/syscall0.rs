#[inline(always)]
pub fn syscall0(n: usize) -> usize {
    let ret: usize;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") n,
            out("rcx") _,
            out("r11") _,
            lateout("rax") ret,
        );
    }
    ret
}
