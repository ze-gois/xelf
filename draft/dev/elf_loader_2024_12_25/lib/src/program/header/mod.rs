use crate::arch;

pub mod ptype;
pub use ptype::Type;

pub mod flag;
pub use flag::Flag;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Header {
    /// Type of segment
    pub ptype: arch::Word,
    /// Segment attributes
    pub flags: arch::Word,
    /// Offset in file
    pub offset: arch::Off,
    /// Virtual address in memory
    pub vaddr: arch::Addr,
    /// Reserved
    pub paddr: arch::Addr,
    /// Size of segment in file
    pub filesz: arch::XWord,
    /// Size of segment in memory
    pub memsz: arch::XWord,
    /// Alignment of segment
    pub align: arch::XWord,
}

impl Header {
    /// Loads ELF Identifier from a filemap
    ///
    /// ```rust
    /// let path : &str = "./data/symver.powerpc64.so";
    /// let program_header = lib::header::Program::read_from_memmap(&map);
    /// println!("{}",program_header);
    /// ```
    pub fn read_nth_from_filepath<P: AsRef<std::path::Path>>(
        filepath: P,
        index: arch::Half,
    ) -> Self {
        let file = std::fs::File::open(filepath).unwrap();
        let filemap = unsafe { memmap2::Mmap::map(&file).unwrap() };
        let elf_header = crate::ELFHeader::read_from_memmap(&filemap);
        Self::read_nth_from_elf_header(&filemap, &elf_header, index)
    }

    /// Loads ELF Identifier from a filemap
    ///
    /// ```rust
    /// let path : &str = "./data/symver.powerpc64.so";
    /// let file = core::fs::File::open(filepath).unwrap();
    /// let filemap = unsafe { memmap2::Mmap::map(&file).unwrap() };
    /// let elf_header = lib::header::ELF::read_from_memmap(&filemap);
    /// let index = elf_header.shstrndx;
    /// let program_header =
    ///     lib::header::Program::read_nth_from_elf_header(&filemap, &elf_header, index);
    /// println!("{}",program_header);
    /// ```
    pub fn read_nth_from_elf_header(
        filemap: &memmap2::Mmap,
        elf_header: &crate::ELFHeader,
        index: arch::Half,
    ) -> Self {
        let mut offset = elf_header.shoff + (index * elf_header.shentsize) as arch::Off;
        let endianess = elf_header.get_identifier().get_endianess();

        Self::read_from_memmap(&filemap, &mut offset, &endianess)
    }

    /// Loads ELF Identifier from a filemap
    ///
    /// ```rust
    /// let path : &str = "./data/symver.powerpc64.so";
    /// let file = core::fs::File::open(filepath).unwrap();
    /// let filemap = unsafe { memmap2::Mmap::map(&file).unwrap() };
    /// let elf_header = lib::header::ELF::read_from_memmap(&filemap);
    /// let program_header =
    ///     lib::header::Program::read_nth_from_elf_header(&filemap, &elf_header, index);
    /// println!("{}",program_header);
    /// ```
    pub fn read_from_memmap(
        filemap: &memmap2::Mmap,
        offset: &mut arch::Off,
        endianess: &arch::Endianess,
    ) -> Self {
        Self {
            ptype: arch::read_and_seek_word(filemap, offset, &endianess),
            flags: arch::read_and_seek_word(filemap, offset, &endianess),
            offset: arch::read_and_seek_off(filemap, offset, &endianess),
            vaddr: arch::read_and_seek_addr(filemap, offset, &endianess),
            paddr: arch::read_and_seek_addr(filemap, offset, &endianess),
            filesz: arch::read_and_seek_xword(filemap, offset, &endianess),
            memsz: arch::read_and_seek_xword(filemap, offset, &endianess),
            align: arch::read_and_seek_xword(filemap, offset, &endianess),
        }
    }

    pub fn copy_ptype(&self) -> arch::Word {
        self.ptype
    }

    pub fn get_ptype(&self) -> Type {
        Type::from(self.copy_ptype())
    }

    // pub fn ptype_as_str(&self) -> &'static str {
    //     self.get_ptype().as_str()
    // }

    pub fn ptype_as_str(&self) -> String {
        format!(
            "{} (PT_{})",
            self.get_ptype().as_str(),
            self.get_ptype().as_acro()
        )
    }

    pub fn copy_flags(&self) -> arch::Word {
        self.flags
    }

    pub fn get_flags(&self) -> Flag {
        Flag::from(self.copy_flags())
    }

    pub fn flag_as_str(&self) -> &'static str {
        self.get_flags().as_str()
    }

    pub fn flag_as_str_acronym(&self) -> &'static str {
        self.get_flags().as_str_acronym()
    }

    pub fn copy_offset(&self) -> arch::Off {
        self.offset
    }

    pub fn copy_vaddr(&self) -> arch::Addr {
        self.vaddr
    }

    pub fn copy_paddr(&self) -> arch::Addr {
        self.paddr
    }

    pub fn copy_filesz(&self) -> arch::XWord {
        self.filesz
    }

    pub fn copy_memsz(&self) -> arch::XWord {
        self.memsz
    }

    pub fn copy_align(&self) -> arch::XWord {
        self.align
    }
}

impl core::fmt::Display for Header {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ProgramHeader{{ ")?;
        write!(f, " Type: {:?} ", self.ptype_as_str())?;
        write!(f, " Flags: {:?} ", self.flag_as_str())?;
        write!(f, " Offset: {:#x} ", self.copy_offset())?;
        write!(f, " VirtAddr: {:#x} ", self.copy_vaddr())?;
        write!(f, " PhysAddr: {:#x} ", self.copy_paddr())?;
        write!(f, " FileSiz: {:#x} ", self.copy_filesz())?;
        write!(f, " MemSiz: {:#x} ", self.copy_memsz())?;
        write!(f, " Align: {:#x}}}", self.copy_align())
    }
}

impl core::fmt::Debug for Header {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Header")
            .field("ptype", &self.ptype_as_str())
            .field("flags", &self.flag_as_str_acronym())
            .field("offset", &self.copy_offset())
            .field("vaddr", &self.copy_vaddr())
            .field("paddr", &self.copy_paddr())
            .field("filesz", &self.copy_filesz())
            .field("memsz", &self.copy_memsz())
            .field("align", &self.copy_align())
            .finish()
    }
}
