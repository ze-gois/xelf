pub type Addr = u32;
pub type Off = u32;
pub type Half = u16;
pub type Word = u32;
pub type SWord = i32;
pub type UnsignedChar = u8;

use crate::arch32::header::elf::identifier::Endianess;

#[derive(Clone, Copy)]
pub enum Type {
    Addr = 0,
    Off = 1,
    Half = 2,
    Word = 3,
    SWord = 4,
    UnsignedChar = 7,
}

impl Type {
    pub fn size(&self) -> Off {
        match self {
            Self::Addr => 32,
            Self::Off => 32,
            Self::Half => 16,
            Self::Word => 32,
            Self::SWord => 32,
            Self::UnsignedChar => 8,
        }
    }
}

pub fn read_addr(filemap: &memmap2::Mmap, offset: Off, endianess: &Endianess) -> Addr {
    let upper_bound = offset + Type::Addr.size() / 8;
    match endianess {
        Endianess::LSB => Addr::from_le_bytes(
            filemap[(offset as usize)..(upper_bound as usize)]
                .try_into()
                .unwrap(),
        ),
        Endianess::MSB => Addr::from_be_bytes(
            filemap[(offset as usize)..(upper_bound as usize)]
                .try_into()
                .unwrap(),
        ),
        _ => panic!("Fail."),
    }
}

pub fn read_off(filemap: &memmap2::Mmap, offset: Off, endianess: &Endianess) -> Off {
    let upper_bound = offset + Type::Off.size() / 8;
    match endianess {
        Endianess::LSB => Off::from_le_bytes(
            filemap[(offset as usize)..(upper_bound as usize)]
                .try_into()
                .unwrap(),
        ),
        Endianess::MSB => Off::from_be_bytes(
            filemap[(offset as usize)..(upper_bound as usize)]
                .try_into()
                .unwrap(),
        ),
        _ => panic!("Fail."),
    }
}

pub fn read_half(filemap: &memmap2::Mmap, offset: Off, endianess: &Endianess) -> Half {
    let upper_bound = offset + Type::Half.size() / 8;
    match endianess {
        Endianess::LSB => Half::from_le_bytes(
            filemap[(offset as usize)..(upper_bound as usize)]
                .try_into()
                .unwrap(),
        ),
        Endianess::MSB => Half::from_be_bytes(
            filemap[(offset as usize)..(upper_bound as usize)]
                .try_into()
                .unwrap(),
        ),
        _ => panic!("Fail."),
    }
}

pub fn read_word(filemap: &memmap2::Mmap, offset: Off, endianess: &Endianess) -> Word {
    let upper_bound = offset + Type::Word.size() / 8;
    match endianess {
        Endianess::LSB => Word::from_le_bytes(
            filemap[(offset as usize)..(upper_bound as usize)]
                .try_into()
                .unwrap(),
        ),
        Endianess::MSB => Word::from_be_bytes(
            filemap[(offset as usize)..(upper_bound as usize)]
                .try_into()
                .unwrap(),
        ),
        _ => panic!("Fail."),
    }
}

pub fn read_sword(filemap: &memmap2::Mmap, offset: Off, endianess: &Endianess) -> SWord {
    let upper_bound = offset + Type::SWord.size() / 8;
    match endianess {
        Endianess::LSB => SWord::from_le_bytes(
            filemap[(offset as usize)..(upper_bound as usize)]
                .try_into()
                .unwrap(),
        ),
        Endianess::MSB => SWord::from_be_bytes(
            filemap[(offset as usize)..(upper_bound as usize)]
                .try_into()
                .unwrap(),
        ),
        _ => panic!("Fail."),
    }
}

pub fn read_unsigned_char(
    filemap: &memmap2::Mmap,
    offset: Off,
    endianess: &Endianess,
) -> UnsignedChar {
    let upper_bound = offset + Type::UnsignedChar.size() / 8;
    match endianess {
        Endianess::LSB => UnsignedChar::from_le_bytes(
            filemap[(offset as usize)..(upper_bound as usize)]
                .try_into()
                .unwrap(),
        ),
        Endianess::MSB => UnsignedChar::from_be_bytes(
            filemap[(offset as usize)..(upper_bound as usize)]
                .try_into()
                .unwrap(),
        ),
        _ => panic!("Fail."),
    }
}

pub fn read_and_seek_addr(
    filemap: &memmap2::Mmap,
    offset: &mut Off,
    endianess: &Endianess,
) -> Addr {
    let value = read_addr(filemap, *offset, endianess);
    *offset = *offset + Type::Addr.size() / 8;
    value
}

pub fn read_and_seek_off(filemap: &memmap2::Mmap, offset: &mut Off, endianess: &Endianess) -> Off {
    let value = read_off(filemap, *offset, endianess);
    *offset = *offset + Type::Off.size() / 8;
    value
}

pub fn read_and_seek_half(
    filemap: &memmap2::Mmap,
    offset: &mut Off,
    endianess: &Endianess,
) -> Half {
    let value = read_half(filemap, *offset, endianess);
    *offset = *offset + Type::Half.size() / 8;
    value
}

pub fn read_and_seek_word(
    filemap: &memmap2::Mmap,
    offset: &mut Off,
    endianess: &Endianess,
) -> Word {
    let value = read_word(filemap, *offset, endianess);
    *offset = *offset + Type::Word.size() / 8;
    value
}

pub fn read_and_seek_sword(
    filemap: &memmap2::Mmap,
    offset: &mut Off,
    endianess: &Endianess,
) -> SWord {
    let value = read_sword(filemap, *offset, endianess);
    *offset = *offset + Type::SWord.size() / 8;
    value
}

pub fn read_and_seek_unsigned_char(
    filemap: &memmap2::Mmap,
    offset: &mut Off,
    endianess: &Endianess,
) -> UnsignedChar {
    let value = read_unsigned_char(filemap, *offset, endianess);
    *offset = *offset + Type::UnsignedChar.size() / 8;
    value
}

pub fn read_and_seek_n_addr(
    filemap: &memmap2::Mmap,
    offset: &mut Off,
    endianess: &Endianess,
    n: Word,
) -> Vec<Addr> {
    (0..n)
        .map(|_| self::read_and_seek_addr(filemap, offset, endianess))
        .collect::<Vec<Addr>>()
}

pub fn read_and_seek_n_off(
    filemap: &memmap2::Mmap,
    offset: &mut Off,
    endianess: &Endianess,
    n: Word,
) -> Vec<Off> {
    (0..n)
        .map(|_| self::read_and_seek_off(filemap, offset, endianess))
        .collect::<Vec<Off>>()
}

pub fn read_and_seek_n_half(
    filemap: &memmap2::Mmap,
    offset: &mut Off,
    endianess: &Endianess,
    n: Word,
) -> Vec<Half> {
    (0..n)
        .map(|_| self::read_and_seek_half(filemap, offset, endianess))
        .collect::<Vec<Half>>()
}

pub fn read_and_seek_n_word(
    filemap: &memmap2::Mmap,
    offset: &mut Off,
    endianess: &Endianess,
    n: Word,
) -> Vec<Word> {
    (0..n)
        .map(|_| self::read_and_seek_word(filemap, offset, endianess))
        .collect::<Vec<Word>>()
}

pub fn read_and_seek_n_sword(
    filemap: &memmap2::Mmap,
    offset: &mut Off,
    endianess: &Endianess,
    n: Word,
) -> Vec<SWord> {
    (0..n)
        .map(|_| self::read_and_seek_sword(filemap, offset, endianess))
        .collect::<Vec<SWord>>()
}

pub fn read_and_seek_n_unsigned_char(
    filemap: &memmap2::Mmap,
    offset: &mut Off,
    endianess: &Endianess,
    n: Word,
) -> Vec<UnsignedChar> {
    (0..n)
        .map(|_| self::read_and_seek_unsigned_char(filemap, offset, endianess))
        .collect::<Vec<UnsignedChar>>()
}
