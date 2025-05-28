#![allow(unused)]
mod callable;
mod stacking;

pub use callable::*;
pub use stacking::*;

pub type _Pointer = *mut u64;
#[derive(Clone, Copy)]
pub struct Pointer(pub _Pointer);

pub type _Byte = *mut u8;
pub struct Byte(pub _Byte);

impl Pointer {
    pub fn current() -> Self {
        let _pointer: _Pointer;
        unsafe { core::arch::asm!("mov {}, rsp", out(reg) _pointer) };
        Pointer(_pointer)
    }
}
