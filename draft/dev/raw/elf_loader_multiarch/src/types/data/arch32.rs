pub enum Data32 {
    Addr(u32),
    Half(u16),
    Off(u32),
    Word(u32),
    SWord(i32),
    UChar(u8),
}

pub type Elf32Addr = u32;
pub type Elf32Half = u16;
pub type Elf32Off = u32;
pub type Elf32Word = u32;
pub type Elf32SWord = i32;
pub type Elf32UChar = u8;
