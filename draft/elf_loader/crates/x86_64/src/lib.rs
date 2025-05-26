#![no_std]
#![allow(warnings)]
pub mod memory;
pub mod print;
pub mod stack;
pub mod syscall;

pub use print::*;
pub use syscall::Syscall;

pub trait ToFlags {
    fn to(self) -> i32;
}
