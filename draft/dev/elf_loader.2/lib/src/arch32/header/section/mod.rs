pub mod flag;
pub mod info;
pub mod link;
pub mod rel;
pub mod rela;
pub mod stype;

pub use info::Info;
pub use link::Link;
pub use stype::Type;

pub use super::super::{data, header};

pub use crate::arch32::table::section::Index;

#[derive(Clone, Copy)]
pub struct Header {
    pub name: data::Word,      /* Section name index*/
    pub stype: data::Word,     /* Section type */
    pub flags: data::Word,     /* Section attributes */
    pub addr: data::Addr,      /* Virtual address in memory */
    pub offset: data::Off,     /* Offset in file */
    pub size: data::Word,      /* Size of section */
    pub link: data::Word,      /* Other section index */
    pub info: data::Word,      /* Miscellaneous information */
    pub addralign: data::Word, /* Address alignment boundary */
    pub entsize: data::Word,   /* Size of entries, if section has table */
}

impl Header {
    pub fn empty() -> Self {
        Self {
            name: 0,                     //  No name
            stype: Type::Null.to(),      //  Inactive
            flags: 0,                    //  No flags
            addr: 0,                     //  No address
            offset: 0,                   //  No file offset
            size: 0,                     //  No size
            link: Index::Undefined.to(), //  No link information
            info: 0,                     //  No auxiliary information
            addralign: 0,                //  No alignment
            entsize: 0,                  //  No entries
        }
    }

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
            name: data::read_and_seek_word(filemap, &mut offset, &endianess),
            stype: data::read_and_seek_word(filemap, &mut offset, &endianess),
            flags: data::read_and_seek_word(filemap, &mut offset, &endianess),
            addr: data::read_and_seek_addr(filemap, &mut offset, &endianess),
            offset: data::read_and_seek_off(filemap, &mut offset, &endianess),
            size: data::read_and_seek_word(filemap, &mut offset, &endianess),
            link: data::read_and_seek_word(filemap, &mut offset, &endianess),
            info: data::read_and_seek_word(filemap, &mut offset, &endianess),
            addralign: data::read_and_seek_word(filemap, &mut offset, &endianess),
            entsize: data::read_and_seek_word(filemap, &mut offset, &endianess),
        }
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

    pub fn get_link(&self) -> Link {
        match self.get_stype() {
            Type::Dynamic => Link::StringTableIndex(self.link),
            Type::Hash => Link::HashTableIndex(self.link),
            Type::Rel => Link::SymbolTableIndex(self.link),
            Type::Rela => Link::SymbolTableIndex(self.link),
            Type::SymTab => Link::OperatingSystem(self.link),
            Type::DynSym => Link::OperatingSystem(self.link),
            _ => Link::Unused(self.link),
        }
    }

    pub fn get_link_string(&self) -> String {
        self.get_link().as_string()
    }

    pub fn get_info(&self) -> Info {
        match self.get_stype() {
            Type::Dynamic => Info::Unused(self.info),
            Type::Hash => Info::Unused(self.info),
            Type::Rel => Info::ReallocationIndex(self.info),
            Type::Rela => Info::ReallocationIndex(self.info),
            Type::SymTab => Info::OperatingSystem(self.info),
            Type::DynSym => Info::OperatingSystem(self.info),
            _ => Info::Unused(self.info),
        }
    }

    pub fn get_info_string(&self) -> String {
        self.get_info().as_string()
    }

    pub fn get_addralign_string(&self) -> String {
        format!("{}", self.addralign)
    }

    pub fn get_entsize_string(&self) -> String {
        format!("{}", self.entsize)
    }
}

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Section Header {{
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
