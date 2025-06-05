use crate::arch;

pub mod flag;
pub use flag::Flag;

pub mod rel;
pub use rel::Rel;

pub mod rela;
pub use rela::Rela;

pub mod stype;
pub use stype::Type;

use super::Index;

/// ELF object files' sections satisfy several conditions.
/// - Every section in an object file has exactly one section header describing it. Section headers may
/// exist that do not have a section.
/// - Each section occupies one contiguous (possibly empty) sequence of bytes within a file.
/// - Sections in a file may not overlap. No byte in a file resides in more than one section.
/// - An object file may have inactive space. The various headers and the sections might not "cover''
/// every byte in an object file. The contents of the inactive data are unspecified
#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Header {
    /// Section name
    ///
    /// offset, in bytes, to the section name, relative to the start of the section name string table
    pub name: arch::Word,
    /// Section type
    ///
    ///
    pub stype: arch::Word,
    /// Flags (Attributes)
    ///
    ///
    pub flags: arch::XWord,
    /// Virtual address in memory
    ///
    ///
    pub addr: arch::Addr,
    /// Offset in file
    ///
    ///
    pub offset: arch::Off,
    /// Size of section
    ///`
    ///
    pub size: arch::XWord,
    /// Link to other section
    ///
    ///
    pub link: arch::Word,
    /// Miscellaneous information
    ///
    ///
    pub info: arch::Word,
    /// Address alignment boundary
    ///
    ///
    pub addralign: arch::XWord,
    /// Size of entries, if section has table
    ///
    ///
    pub entsize: arch::XWord,
}

impl Header {
    /// # Empty section
    ///
    /// The first entry in the section header table (with an index of 0) is reserved, and
    /// must contain all zeroes.
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

    pub fn read_nth_from_filepath<P: AsRef<std::path::Path>>(
        filepath: P,
        index: arch::Half,
    ) -> Self {
        let file = std::fs::File::open(filepath).unwrap();
        let filemap = unsafe { memmap2::Mmap::map(&file).unwrap() };
        let elf_header = crate::ELFHeader::read_from_memmap(&filemap);
        Self::read_nth_from_elf_header(&filemap, &elf_header, index)
    }

    pub fn read_nth_from_elf_header(
        filemap: &memmap2::Mmap,
        elf_header: &crate::ELFHeader,
        index: arch::Half,
    ) -> Self {
        let mut offset = elf_header.shoff + (index * elf_header.shentsize) as arch::Off;
        let endianess = elf_header.get_identifier().get_endianess();

        Self::read_from_memmap(&filemap, &mut offset, &endianess)
    }

    pub fn read_from_memmap(
        filemap: &memmap2::Mmap,
        offset: &mut arch::Off,
        endianess: &arch::Endianess,
    ) -> Self {
        Self {
            name: arch::read_and_seek_word(filemap, offset, &endianess),
            stype: arch::read_and_seek_word(filemap, offset, &endianess),
            flags: arch::read_and_seek_xword(filemap, offset, &endianess),
            addr: arch::read_and_seek_addr(filemap, offset, &endianess),
            offset: arch::read_and_seek_off(filemap, offset, &endianess),
            size: arch::read_and_seek_xword(filemap, offset, &endianess),
            link: arch::read_and_seek_word(filemap, offset, &endianess),
            info: arch::read_and_seek_word(filemap, offset, &endianess),
            addralign: arch::read_and_seek_xword(filemap, offset, &endianess),
            entsize: arch::read_and_seek_xword(filemap, offset, &endianess),
        }
    }

    pub fn copy_name(&self) -> arch::Word {
        self.name
    }

    pub fn get_name_from_filemap(
        &self,
        filemap: &memmap2::Mmap,
        table: &crate::StringTable,
        endianess: &crate::arch::Endianess,
    ) -> String {
        let mut string_offset = table.offset + self.name as arch::Off;
        let mut string = String::new();
        while filemap[string_offset as usize] != 0 {
            string.push(
                arch::read_and_seek_unsigned_char(filemap, &mut string_offset, endianess) as char,
            );
        }
        string
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

    pub fn get_flag(&self) -> Flag {
        Flag::from(self.flags)
    }

    pub fn get_flag_str(&self) -> &'static str {
        self.get_flag().as_str()
    }

    pub fn get_flag_str_acronym(&self) -> &'static str {
        self.get_flag().as_str_acronym()
    }
}

impl std::fmt::Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "tSection Header:")?;
        write!(f, "\tname: {:?}", self.get_name_string())?;
        write!(f, "\tstype: {:?}", self.get_stype_str())?;
        write!(f, "\tflags: {:?}", self.get_flag_str_acronym())?;
        write!(f, "\taddr: {:#x}", self.addr)?;
        write!(f, "\toffset: {:#x}", self.offset)?;
        write!(f, "\tsize: {:#x}", self.size)?;
        write!(f, "\tlink: {:#x}", self.link)?;
        write!(f, "\tinfo: {:#x}", self.info)?;
        write!(f, "\taddralign: {:#x}", self.addralign)?;
        write!(f, "\tentsize: {:#x}", self.entsize)?;
        Ok(())
    }
}

impl std::fmt::Debug for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Header {{ name: {}, stype: {}, flags: {}, addr: {}, offset: {}, size: {}, link: {}, info: {}, addralign: {}, entsize: {} }}",
            self.name,
            self.stype,
            self.flags,
            self.addr,
            self.offset,
            self.size,
            self.link,
            self.info,
            self.addralign,
            self.entsize
        )
    }
}
