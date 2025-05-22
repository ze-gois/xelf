#![no_std]
#![allow(warnings)]
pub mod memory;
pub mod misc;
pub mod syscall;

pub use misc::print::*;
pub use syscall::Syscall;

pub trait ToFlags {
    fn to(self) -> i32;
}
