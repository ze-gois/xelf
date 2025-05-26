use crate::arch64::{data, header};

#[derive(Clone, Copy)]
pub struct Header {
    pub ptype: data::Word,
    pub flags: data::Word,
    pub offset: data::Off,
    pub vaddr: data::Addr,
    pub paddr: data::Addr,
    pub filesz: data::XWord,
    pub memsz: data::XWord,
    pub align: data::XWord,
}

impl Header {
    pub fn load_from_filepath<P: AsRef<std::path::Path>>(filepath: P) -> Self {
        let file = std::fs::File::open(filepath).unwrap();
        let filemap = unsafe { memmap2::Mmap::map(&file).unwrap() };
        let elf_header = header::ELF::load_from_memmap(&filemap);

        let mut offset = elf_header.get_phoff();
        let endianess = elf_header.get_identifier().get_endianess();

        Self::load_from_memmap(&filemap, &mut offset, &endianess)
    }

    pub fn load_from_memmap(
        filemap: &memmap2::Mmap,
        offset: &mut data::Off,
        endianess: &header::elf::identifier::Endianess,
    ) -> Self {
        let mut offset = offset;

        Self {
            ptype: data::read_and_seek_word(filemap, &mut offset, &endianess),
            flags: data::read_and_seek_word(filemap, &mut offset, &endianess),
            offset: data::read_and_seek_off(filemap, &mut offset, &endianess),
            vaddr: data::read_and_seek_addr(filemap, &mut offset, &endianess),
            paddr: data::read_and_seek_addr(filemap, &mut offset, &endianess),
            filesz: data::read_and_seek_xword(filemap, &mut offset, &endianess),
            memsz: data::read_and_seek_xword(filemap, &mut offset, &endianess),
            align: data::read_and_seek_xword(filemap, &mut offset, &endianess),
        }
    }
}
