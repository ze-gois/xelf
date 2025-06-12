use crate::dtype;

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
    /// offset - in bytes - to the section name, relative to the start of the section name string table
    pub name: dtype::Word,
    /// Section type
    ///
    ///
    pub stype: dtype::Word,
    /// Flags (Attributes)
    ///
    ///
    pub flags: dtype::XWord,
    /// Virtual address in memory
    ///
    ///
    pub addr: dtype::Addr,
    /// Offset in file
    ///
    ///
    pub offset: dtype::Off,
    /// Size of section
    ///`
    ///
    pub size: dtype::XWord,
    /// Link to other section
    ///
    ///
    pub link: dtype::Word,
    /// Miscellaneous information
    ///
    ///
    pub info: dtype::Word,
    /// Address alignment boundary
    ///
    ///
    pub addralign: dtype::XWord,
    /// Size of entries, if section has table
    ///
    ///
    pub entsize: dtype::XWord,
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

    pub fn nth_from_filepath(filepath: &str, index: usize) -> Self {
        let file_descriptor = crate::open_filepath(filepath);
        let elf_header = crate::ELFHeader::from_file_descriptor(file_descriptor);
        Self::nth_from_elf_header(file_descriptor, &elf_header, index)
    }

    pub fn nth_from_elf_header(
        file_descriptor: isize,
        elf_header: &crate::ELFHeader,
        index: usize,
    ) -> Self {
        let endianess = elf_header.get_identifier().get_endianess();

        let mut offset = elf_header.shoff + (index * elf_header.shentsize.0);
        syscall::lseek(file_descriptor, offset, syscall::lseek::Flag::SET.into());

        Self::from_file_descriptor(file_descriptor, &endianess)
    }

    pub fn from_file_descriptor(
        file_descriptor: isize,
        endianess: dtype::Endianness,
    ) -> crate::Result<Self> {
        Ok(Self {
            name: dtype::Word::read(file_descriptor, endianess)?,
            stype: dtype::Word::read(file_descriptor, endianess)?,
            flags: dtype::XWord::read(file_descriptor, endianess)?,
            addr: dtype::Addr::read(file_descriptor, endianess)?,
            offset: dtype::Off::read(file_descriptor, endianess)?,
            size: dtype::XWord::read(file_descriptor, endianess)?,
            link: dtype::Word::read(file_descriptor, endianess)?,
            info: dtype::Word::read(file_descriptor, endianess)?,
            addralign: dtype::XWord::read(file_descriptor, endianess)?,
            entsize: dtype::XWord::read(file_descriptor, endianess)?,
        })
    }

    pub fn copy_name(&self) -> dtype::Word {
        self.name
    }

    pub fn get_name_from_file_descriptor(
        &self,
        file_descriptor: isize,
        table: &crate::StringTable,
        endianness: &dtype::Endianness,
    ) -> &str {
        let mut string_offset = table.offset + self.name.0;

        syscall::lseek(
            file_descriptor,
            string_offset,
            syscall::lseek::Flag::SET.into(),
        );

        let mut counter = 0;

        loop {
            let char = dtype::UChar::read(file_descriptor, endianness)?;
            if char.0 == 0 {
                break;
            }
            counter = counter + 1;
        }

        let pointer = crate::alloc::<u8>(counter)?;

        let mut counter = 0;

        loop {
            let char = dtype::UChar::read(file_descriptor, endianness)?;
            unsafe { *pointer.add(counter as isize) = char.0 };
            if char.0 == 0 {
                break;
            }
            counter = counter + 1;
        }

        let slice = unsafe { core::slice::from_raw_parts(pointer, length) };
        let string = core::str::from_utf8(slice).unwrap_or("");

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

impl core::fmt::Display for Header {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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

impl core::fmt::Debug for Header {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
