pub mod flag;
pub mod rel;
pub mod rela;
pub mod stype;

pub use flag::*;
pub use rel::*;
pub use rela::*;
pub use stype::*;

use crate::arch64::data;
use crate::arch64::header;

#[derive(Clone, Copy)]
pub struct Header {
    map_offset: data::Off,
    name: data::Word,       /* Section name */
    stype: data::Word,      /* Section type */
    flags: data::XWord,     /* Section attributes */
    addr: data::Addr,       /* Virtual address in memory */
    offset: data::Off,      /* Offset in file */
    size: data::XWord,      /* Size of section */
    link: data::Word,       /* Link to other section */
    info: data::Word,       /* Miscellaneous information */
    addralign: data::XWord, /* Address alignment boundary */
    entsize: data::XWord,   /* Size of entries, if section has table */
}

impl Header {
    pub fn load_from_filepath<P: AsRef<std::path::Path>>(filepath: P) -> Self {
        let file = std::fs::File::open(filepath).unwrap();
        let filemap = unsafe { memmap2::Mmap::map(&file).unwrap() };
        let elf_header = header::ELF::load_from_memmap(&filemap);

        let mut offset = elf_header.get_shoff();
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
            map_offset: *offset,
            name: data::read_and_seek_word(filemap, &mut offset, &endianess),
            stype: data::read_and_seek_word(filemap, &mut offset, &endianess),
            flags: data::read_and_seek_xword(filemap, &mut offset, &endianess),
            addr: data::read_and_seek_addr(filemap, &mut offset, &endianess),
            offset: data::read_and_seek_off(filemap, &mut offset, &endianess),
            size: data::read_and_seek_xword(filemap, &mut offset, &endianess),
            link: data::read_and_seek_word(filemap, &mut offset, &endianess),
            info: data::read_and_seek_word(filemap, &mut offset, &endianess),
            addralign: data::read_and_seek_xword(filemap, &mut offset, &endianess),
            entsize: data::read_and_seek_xword(filemap, &mut offset, &endianess),
        }
    }

    pub fn copy_map_offset(&self) -> data::Off {
        self.map_offset
    }

    pub fn copy_name(&self) -> data::Word {
        self.name
    }

    pub fn copy_stype(&self) -> data::Word {
        self.stype
    }

    pub fn copy_flags(&self) -> data::XWord {
        self.flags
    }

    pub fn copy_addr(&self) -> data::Addr {
        self.addr
    }

    pub fn copy_offset(&self) -> data::Off {
        self.offset
    }

    pub fn copy_size(&self) -> data::XWord {
        self.size
    }

    pub fn copy_link(&self) -> data::Word {
        self.link
    }

    pub fn copy_info(&self) -> data::Word {
        self.info
    }

    pub fn copy_addralign(&self) -> data::XWord {
        self.addralign
    }

    pub fn copy_entsize(&self) -> data::XWord {
        self.entsize
    }

    pub fn get_map_offset_string(&self) -> String {
        format!("{}", self.map_offset)
    }

    pub fn get_name_string(&self) -> String {
        format!("{}", self.name)
    }

    pub fn get_stype(&self) -> Type {
        Type::from(self.stype)
    }

    pub fn get_stype_str(&self) -> &str {
        self.get_stype().as_str()
    }

    pub fn get_flags_string(&self) -> String {
        format!("{}", self.flags)
    }

    pub fn get_addr_string(&self) -> String {
        format!("{}", self.addr)
    }

    pub fn get_offset_string(&self) -> String {
        format!("{}", self.offset)
    }

    pub fn get_size_string(&self) -> String {
        format!("{}", self.size)
    }

    pub fn get_link_string(&self) -> String {
        format!("{}", self.link)
    }

    pub fn get_info_string(&self) -> String {
        format!("{}", self.info)
    }

    pub fn get_addralign_string(&self) -> String {
        format!("{}", self.addralign)
    }

    pub fn get_entsize_string(&self) -> String {
        format!("{}", self.entsize)
    }
}

impl core::fmt::Display for Header {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Section Header {{
                Map offset: {},
                Name: {},
                Type: {},
                Attributes: {},
                Address: {},
                Offset: {},
                Size: {},
                Link: {},
                Information: {},
                Alignment: {},
                Entry size: {},
            }}",
            self.get_map_offset_string(),
            self.get_name_string(),
            self.get_stype_str(),
            self.get_flags_string(),
            self.get_addr_string(),
            self.get_offset_string(),
            self.get_size_string(),
            self.get_link_string(),
            self.get_info_string(),
            self.get_addralign_string(),
            self.get_entsize_string(),
        )
    }
}
