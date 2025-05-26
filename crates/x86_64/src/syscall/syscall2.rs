use crate::result::{Result, handle_result};

#[inline(always)]
pub fn syscall2(n: usize, a1: usize, a2: usize) -> Result<isize> {
    let ret: usize;
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") n => ret,
            in("rdi") a1,
            in("rsi") a2,
            out("rcx") _,
            out("r11") _,

        );
    }
    handle_result(ret)
}
