use super::define_type;

use crate::error::{Error, Result};
use x86_64::syscall;

define_type!(pub XWord, u64);
define_type!(pub SWord, i32);
define_type!(pub SXWord, i64);
define_type!(pub Off, u64);
define_type!(pub Word, u32);
define_type!(pub Half, u16);
define_type!(pub Addr, u64);
define_type!(pub UChar, u8);

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Endianness {
    Little,
    Big,
}

pub static mut ENDIANNESS: core::mem::MaybeUninit<Endianness> = core::mem::MaybeUninit::uninit();

pub unsafe fn set_endianness(e: Endianness) {
    ENDIANNESS.write(e);
}

pub unsafe fn get_endianness() -> Endianness {
    *ENDIANNESS.assume_init_ref()
}

pub trait ELFType {
    const SIZE_BITS: usize;
    const SIZE_BYTES: usize = Self::SIZE_BITS / 8;
}

// // Generic trait for reading types from fd
// pub trait ReadFromFd: Sized {
//     unsafe fn read_from(fd: i32) -> Result<Self, i64>;
// }

// // Implementation for base types
// impl ReadFromFd for u16 {
//     unsafe fn read_from(fd: i32) -> Result<Self, i64> {
//         let mut bytes = [0u8; 2];
//         match x86_64::syscall::read(fd, bytes.as_mut_ptr(), 2) {
//             Ok(2) => {
//                 let value = match *ENDIANNESS.assume_init_ref() {
//                     Endianness::Little => u16::from_le_bytes(bytes),
//                     Endianness::Big => u16::from_be_bytes(bytes),
//                 };
//                 Ok(value)
//             },
//             Ok(_) => Err(-1),  // Short read
//             Err(e) => Err(e as i64),
//         }
//     }
// }
