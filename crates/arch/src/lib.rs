#![no_std]
#![allow(unused)]
pub mod macros;
pub mod memory;
pub mod traits;

pub use human::info;

#[cfg(target_arch = "x86_64")]
pub use x86_64 as arch;
#[cfg(target_arch = "x86_64")]
pub mod _x86_64;
#[cfg(target_arch = "x86_64")]
pub use _x86_64 as _arch;

pub use _arch::*;
pub use arch::*;

pub use traits::*;

pub struct Arch;
