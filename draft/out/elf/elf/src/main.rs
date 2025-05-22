#![no_std]
#![no_main]
#![feature(naked_functions)]
#![allow(unused)]

mod arch;
// mod dtype;
mod error;
// mod macros;
// mod memory;
// mod print;
mod syscall;

// use arch::*;
use error::*;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    // won't ever get executed anyway
    loop {}
    // unsafe { syscall::write(1, aux.as_ptr(), aux.len()) };
    // unsafe { syscall::exit(1) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn entry(sp: *const usize) -> ! {
    unsafe {
        let msg = b"Debug: Write syscall called\n";
        let mut ret: usize;
        core::arch::asm!(
            "syscall",
            in("rax") 1usize,
            in("rdi") 2usize, // stderr
            in("rsi") msg.as_ptr(),
            in("rdx") msg.len(),
            out("rcx") _,
            out("r11") _,
            lateout("rax") ret,
        );
    }

    loop {}
    // let aux = "Entry at work\n";
    // unsafe {
    //     let _ = syscall::write(1, aux.as_ptr(), aux.len());
    // };
    // unsafe { syscall::exit(0) }
}

pub extern "C" fn exit() -> ! {
    loop {}
    // let aux = "Fini at work\n";
    // unsafe {
    //     let _ = syscall::write(1, aux.as_ptr(), aux.len());
    // };
}

#[cfg(test)]
mod tests {
    // Your tests that use std
}
