use crate::result::{Result, handle_result};

use human::info;

#[inline(always)]
pub fn syscall3(n: usize, a1: usize, a2: usize, a3: usize) -> Result<isize> {
    let ret: usize;
    info!("crate: syscall3: \"");
    unsafe {
        core::arch::asm!(
            "syscall",
            inlateout("rax") n => ret,
            in("rdi") a1,
            in("rsi") a2,
            in("rdx") a3,
            out("rcx") _,
            out("r11") _,

        );
    }
    info!("\" .. done\n");
    handle_result(ret)
}
