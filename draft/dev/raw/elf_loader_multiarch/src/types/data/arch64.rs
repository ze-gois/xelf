pub enum Data64 {
    Addr(u64),
    Off(u64),
    Half(u16),
    Word(u32),
    SWord(i32),
    XWord(u64),
    SXWord(i64),
    UChar(u8),
}

pub type Elf64Addr = u64;
pub type Elf64Off = u64;
pub type Elf64Half = u16;
pub type Elf64Word = u32;
pub type Elf64SWord = i32;
pub type Elf64XWord = u64;
pub type Elf64SXWord = i64;
pub type Elf64UChar = u8;
