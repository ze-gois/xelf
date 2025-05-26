use crate::result::{Result, handle_result};

#[inline(always)]
pub fn syscall0(n: usize) -> Result<isize> {
    let syscall_return: usize;
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") n => syscall_return,
            out("rcx") _,
            out("r11") _,
        );
    }
    handle_result(syscall_return)
}
