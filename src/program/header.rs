use crate::dtype;

pub mod ptype;
pub use ptype::Type;

pub mod flag;
pub use flag::Flag;
use syscall::lseek;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Header {
    /// Type of segment
    pub ptype: dtype::Word,
    /// Segment attributes
    pub flags: dtype::Word,
    /// Offset in file
    pub offset: dtype::Off,
    /// Virtual address in memory
    pub vaddr: dtype::Addr,
    /// Reserved
    pub paddr: dtype::Addr,
    /// Size of segment in file
    pub filesz: dtype::XWord,
    /// Size of segment in memory
    pub memsz: dtype::XWord,
    /// Alignment of segment
    pub align: dtype::XWord,
}

impl Header {
    /// Loads ELF Identifier from a filemap
    ///
    /// ```rust
    /// let path : &str = "./data/symver.powerpc64.so";
    /// let program_header = lib::header::Program::read_from_memmap(&map);
    /// println!("{}",program_header);
    /// ```
    pub fn read_nth_from_filepath(filepath: &str, index: dtype::Half) -> Self {
        let file_descriptor = crate::open_filepath(filepath);
        let elf_header = crate::ELFHeader::read_from_file_descriptor(file_descriptor);
        Self::read_nth_from_elf_header(file_descriptor, &elf_header, index)
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
        file_descriptor: isize,
        elf_header: &crate::ELFHeader,
        index: dtype::Half,
    ) -> crate::Result<Self> {
        let offset = elf_header.shoff.0 + (index.0 * elf_header.shentsize.0) as u64;

        syscall::lseek(file_descriptor as i32, offset as i64, lseek::Flag::SET.to());

        let endianness = elf_header.get_identifier().get_endianness();

        Self::read_from_file_descriptor(file_descriptor, endianness)
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
    pub fn read_from_file_descriptor(
        file_descriptor: isize,
        endianness: dtype::Endianness,
    ) -> crate::Result<Self> {
        Ok(Self {
            ptype: dtype::Word::read(file_descriptor, endianness)?,
            flags: dtype::Word::read(file_descriptor, endianness)?,
            offset: dtype::Off::read(file_descriptor, endianness)?,
            vaddr: dtype::Addr::read(file_descriptor, endianness)?,
            paddr: dtype::Addr::read(file_descriptor, endianness)?,
            filesz: dtype::XWord::read(file_descriptor, endianness)?,
            memsz: dtype::XWord::read(file_descriptor, endianness)?,
            align: dtype::XWord::read(file_descriptor, endianness)?,
        })
    }

    pub fn copy_ptype(&self) -> dtype::Word {
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

    pub fn copy_flags(&self) -> dtype::Word {
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
}

impl core::fmt::Display for Header {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "ProgramHeader{{ ")?;
        write!(f, " Type: {:?} ", self.ptype_as_str())?;
        write!(f, " Flags: {:?} ", self.flag_as_str())?;
        write!(f, " Offset: {:#x} ", self.offset.0)?;
        write!(f, " VirtAddr: {:#x} ", self.vaddr.0)?;
        write!(f, " PhysAddr: {:#x} ", self.paddr.0)?;
        write!(f, " FileSiz: {:#x} ", self.filesz.0)?;
        write!(f, " MemSiz: {:#x} ", self.memsz.0)?;
        write!(f, " Align: {:#x}}}", self.align.0)
    }
}

impl core::fmt::Debug for Header {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Header")
            .field("ptype", &self.ptype_as_str())
            .field("flags", &self.flag_as_str_acronym())
            .field("offset", &self.offset.0)
            .field("vaddr", &self.vaddr.0)
            .field("paddr", &self.paddr.0)
            .field("filesz", &self.filesz.0)
            .field("memsz", &self.memsz.0)
            .field("align", &self.align.0)
            .finish()
    }
}
