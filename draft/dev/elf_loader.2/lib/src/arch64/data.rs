pub type Addr = u64;
pub type Off = u64;
pub type Half = u16;
pub type Word = u32;
pub type SWord = i32;
pub type XWord = u64;
pub type SXWord = i64;
pub type UnsignedChar = u8;

use crate::arch64::header::elf::identifier::Endianess;

#[derive(Clone, Copy)]
pub enum Type {
    Addr = 0,
    Off = 1,
    Half = 2,
    Word = 3,
    SWord = 4,
    XWord = 5,
    SXWord = 6,
    UnsignedChar = 7,
}

impl Type {
    pub fn size(&self) -> Off {
        match self {
            Self::Addr => 64,
            Self::Off => 64,
            Self::Half => 16,
            Self::Word => 32,
            Self::SWord => 32,
            Self::XWord => 64,
            Self::SXWord => 64,
            Self::UnsignedChar => 8,
        }
    }
}

pub fn read_and_seek_addr(
    filemap: &memmap2::Mmap,
    offset: &mut Off,
    endianess: &Endianess,
) -> Addr {
    let upper_bound = *offset + Type::Addr.size() / 8;
    let value = match endianess {
        Endianess::LSB => Addr::from_le_bytes(
            filemap[(*offset as usize)..(upper_bound as usize)]
                .try_into()
                .unwrap(),
        ),
        Endianess::MSB => Addr::from_be_bytes(
            filemap[(*offset as usize)..(upper_bound as usize)]
                .try_into()
                .unwrap(),
        ),
        _ => panic!("Fail."),
    };
    *offset = upper_bound;
    value
}

pub fn read_and_seek_off(filemap: &memmap2::Mmap, offset: &mut Off, endianess: &Endianess) -> Off {
    let upper_bound = *offset + Type::Off.size() / 8;
    let value = match endianess {
        Endianess::LSB => Off::from_le_bytes(
            filemap[(*offset as usize)..(upper_bound as usize)]
                .try_into()
                .unwrap(),
        ),
        Endianess::MSB => Off::from_be_bytes(
            filemap[(*offset as usize)..(upper_bound as usize)]
                .try_into()
                .unwrap(),
        ),
        _ => panic!("Fail."),
    };
    *offset = upper_bound;
    value
}

pub fn read_and_seek_half(
    filemap: &memmap2::Mmap,
    offset: &mut Off,
    endianess: &Endianess,
) -> Half {
    let upper_bound = *offset + Type::Half.size() / 8;
    let value = match endianess {
        Endianess::LSB => Half::from_le_bytes(
            filemap[(*offset as usize)..(upper_bound as usize)]
                .try_into()
                .unwrap(),
        ),
        Endianess::MSB => Half::from_be_bytes(
            filemap[(*offset as usize)..(upper_bound as usize)]
                .try_into()
                .unwrap(),
        ),
        _ => panic!("Fail."),
    };
    *offset = upper_bound;
    value
}

pub fn read_and_seek_word(
    filemap: &memmap2::Mmap,
    offset: &mut Off,
    endianess: &Endianess,
) -> Word {
    let upper_bound = *offset + Type::Word.size() / 8;
    let value = match endianess {
        Endianess::LSB => Word::from_le_bytes(
            filemap[(*offset as usize)..(upper_bound as usize)]
                .try_into()
                .unwrap(),
        ),
        Endianess::MSB => Word::from_be_bytes(
            filemap[(*offset as usize)..(upper_bound as usize)]
                .try_into()
                .unwrap(),
        ),
        _ => panic!("Fail."),
    };
    *offset = upper_bound;
    value
}

pub fn read_and_seek_sword(
    filemap: &memmap2::Mmap,
    offset: &mut Off,
    endianess: &Endianess,
) -> SWord {
    let upper_bound = *offset + Type::SWord.size() / 8;
    let value = match endianess {
        Endianess::LSB => SWord::from_le_bytes(
            filemap[(*offset as usize)..(upper_bound as usize)]
                .try_into()
                .unwrap(),
        ),
        Endianess::MSB => SWord::from_be_bytes(
            filemap[(*offset as usize)..(upper_bound as usize)]
                .try_into()
                .unwrap(),
        ),
        _ => panic!("Fail."),
    };
    *offset = upper_bound;
    value
}

pub fn read_and_seek_xword(
    filemap: &memmap2::Mmap,
    offset: &mut Off,
    endianess: &Endianess,
) -> XWord {
    let upper_bound = *offset + Type::XWord.size() / 8;
    let value = match endianess {
        Endianess::LSB => XWord::from_le_bytes(
            filemap[(*offset as usize)..(upper_bound as usize)]
                .try_into()
                .unwrap(),
        ),
        Endianess::MSB => XWord::from_be_bytes(
            filemap[(*offset as usize)..(upper_bound as usize)]
                .try_into()
                .unwrap(),
        ),
        _ => panic!("Fail."),
    };
    *offset = upper_bound;
    value
}

pub fn read_and_seek_sxword(
    filemap: &memmap2::Mmap,
    offset: &mut Off,
    endianess: &Endianess,
) -> SXWord {
    let upper_bound = *offset + Type::SXWord.size() / 8;
    let value = match endianess {
        Endianess::LSB => SXWord::from_le_bytes(
            filemap[(*offset as usize)..(upper_bound as usize)]
                .try_into()
                .unwrap(),
        ),
        Endianess::MSB => SXWord::from_be_bytes(
            filemap[(*offset as usize)..(upper_bound as usize)]
                .try_into()
                .unwrap(),
        ),
        _ => panic!("Fail."),
    };
    *offset = upper_bound;
    value
}

pub fn read_and_seek_unsigned_char(
    filemap: &memmap2::Mmap,
    offset: &mut Off,
    endianess: &Endianess,
) -> UnsignedChar {
    let upper_bound = *offset + Type::UnsignedChar.size() / 8;
    let value = match endianess {
        Endianess::LSB => UnsignedChar::from_le_bytes(
            filemap[(*offset as usize)..(upper_bound as usize)]
                .try_into()
                .unwrap(),
        ),
        Endianess::MSB => UnsignedChar::from_be_bytes(
            filemap[(*offset as usize)..(upper_bound as usize)]
                .try_into()
                .unwrap(),
        ),
        _ => panic!("Fail."),
    };
    *offset = upper_bound;
    value
}

pub fn read_and_seek_n_addr(
    filemap: &memmap2::Mmap,
    offset: &mut Off,
    endianess: &Endianess,
    n: usize,
) -> Vec<Addr> {
    (0..n)
        .map(|_| self::read_and_seek_addr(filemap, offset, endianess))
        .collect::<Vec<Addr>>()
}

pub fn read_and_seek_n_off(
    filemap: &memmap2::Mmap,
    offset: &mut Off,
    endianess: &Endianess,
    n: usize,
) -> Vec<Off> {
    (0..n)
        .map(|_| self::read_and_seek_off(filemap, offset, endianess))
        .collect::<Vec<Off>>()
}

pub fn read_and_seek_n_half(
    filemap: &memmap2::Mmap,
    offset: &mut Off,
    endianess: &Endianess,
    n: usize,
) -> Vec<Half> {
    (0..n)
        .map(|_| self::read_and_seek_half(filemap, offset, endianess))
        .collect::<Vec<Half>>()
}

pub fn read_and_seek_n_word(
    filemap: &memmap2::Mmap,
    offset: &mut Off,
    endianess: &Endianess,
    n: usize,
) -> Vec<Word> {
    (0..n)
        .map(|_| self::read_and_seek_word(filemap, offset, endianess))
        .collect::<Vec<Word>>()
}

pub fn read_and_seek_n_sword(
    filemap: &memmap2::Mmap,
    offset: &mut Off,
    endianess: &Endianess,
    n: usize,
) -> Vec<SWord> {
    (0..n)
        .map(|_| self::read_and_seek_sword(filemap, offset, endianess))
        .collect::<Vec<SWord>>()
}

pub fn read_and_seek_n_xword(
    filemap: &memmap2::Mmap,
    offset: &mut Off,
    endianess: &Endianess,
    n: usize,
) -> Vec<XWord> {
    (0..n)
        .map(|_| self::read_and_seek_xword(filemap, offset, endianess))
        .collect::<Vec<XWord>>()
}

pub fn read_and_seek_n_sxword(
    filemap: &memmap2::Mmap,
    offset: &mut Off,
    endianess: &Endianess,
    n: usize,
) -> Vec<SXWord> {
    (0..n)
        .map(|_| self::read_and_seek_sxword(filemap, offset, endianess))
        .collect::<Vec<SXWord>>()
}

pub fn read_and_seek_n_unsigned_char(
    filemap: &memmap2::Mmap,
    offset: &mut Off,
    endianess: &Endianess,
    n: usize,
) -> Vec<UnsignedChar> {
    (0..n)
        .map(|_| self::read_and_seek_unsigned_char(filemap, offset, endianess))
        .collect::<Vec<UnsignedChar>>()
}
