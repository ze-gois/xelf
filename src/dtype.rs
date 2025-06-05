use crate::Result;
use crate::elf_define_type;
use crate::impl_from_for_type;
use crate::impl_partial_eq_for_type;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Endianness {
    Little,
    Big,
}

pub static mut ENDIANNESS: core::mem::MaybeUninit<Endianness> = core::mem::MaybeUninit::uninit();

pub fn set_endianness(e: Endianness) {
    unsafe { ENDIANNESS.write(e) };
}

pub fn get_endianness() -> Endianness {
    unsafe { *ENDIANNESS.assume_init_ref() }
}
pub trait ELFType {
    type Inner;
    const SIZE_BITS: usize;
    const SIZE_BYTES: usize = Self::SIZE_BITS / 8;
}

elf_define_type!(pub SXWord, i64); //Unsigned program address
elf_define_type!(pub UChar, u8); //Unsigned file offset
elf_define_type!(pub Half, u16); //Unsigned medium integer
elf_define_type!(pub SWord, i32); //Unsigned integer
elf_define_type!(pub XWord, u64); //Signed integer
elf_define_type!(pub Word, u32); //Unsigned long integer
elf_define_type!(pub Off, u64); //Signed long integer
elf_define_type!(pub Addr, u64); //Unsigned small integer

#[derive(Debug)]
pub enum Error {
    InvalidData,
    InvalidEndian,
    InvalidType,
    ShorterData,
}
