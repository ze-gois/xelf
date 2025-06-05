pub use crate::arch;

/// # ELF Identifier
///
/// ELF provides an object file framework to support multiple processors,
/// multiple data encodings, and multiple classes of machines. To support this object file family,
/// the initial bytes of the file specify how to interpret the file, independent of the processor on
/// which the inquiry is made and independent of the file's remaining contents.
/// The initial bytes of an ELF header (and an object file) correspond to the e_ident member.
pub mod identifier;
pub use identifier::Identifier;

pub mod etype;
pub use etype::Type;

pub mod machine;
pub use machine::Machine;

pub mod version;
pub use version::Version;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Header {
    /// # Identifier to file as an ELF object file
    ///
    /// Provide information about the data representation of the object file structures.
    /// The bytes of this array that have defined meanings are detailed below.
    /// The remaining bytes are reserved for future use, and should be set to zero.
    pub identifier: Identifier,
    /// # Object file type.
    ///
    /// |Variant|Descriminator|Meaning|
    /// |:--|:-:|:--|
    /// |Type::None|0|No file type|
    /// |Type::Rel|1|Relocatable object file|
    /// |Type::Exec|2|Executable file|
    /// |Type::Dyn|3|Shared object file|
    /// |Type::Core|4|Core file|
    /// |Type::LoOS|0xFE00|Environment-specific use lower boundary|
    /// |Type::HiOS|0xFEFF|Environment-specific use upper boundary|
    /// |Type::LoProc|0xFF00|Processor-specific use lower boundary|
    /// |Type::HiProc|0xFFFF|Processor-specific use upper boundary|
    pub etype: arch::Half,
    /// # Machine type
    ///
    /// Target architecture defined in the processor-specific supplements.
    pub machine: arch::Half,
    /// # Object file version
    ///
    /// Currently, this field has the value 1 (EV_CURRENT).
    pub version: arch::Word,
    /// # Entry point address
    ///
    /// Virtual address of the program entry point.
    /// If there is no entry point, this field contains zero.
    pub entry: arch::Addr,
    /// # Program header offset
    ///
    /// In bytes
    pub phoff: arch::Off,
    /// # Section header offset
    ///
    /// The offset is expressed in bytes from the beginning of the ELF object file
    pub shoff: arch::Off,
    /// # Processor-specific flags
    ///
    ///
    pub flags: arch::Word,
    /// # ELF header size
    ///
    ///
    pub ehsize: arch::Half,
    /// # Size of program header entry
    ///
    ///
    pub phentsize: arch::Half,
    /// # Number of program header entries
    ///
    ///
    pub phnum: arch::Half,
    /// # Size of section header entry
    ///
    ///
    pub shentsize: arch::Half,
    /// # Number of section header entries
    ///
    ///
    pub shnum: arch::Half,
    /// # Section name string table index
    ///
    /// Index whithin the Section Header Table.
    /// Despite being a arch::Half it is used as usize.
    pub shstrndx: arch::Half,
}

impl Header {
    pub fn read_from_filepath<P: AsRef<std::path::Path>>(filepath: P) -> Self {
        let file = std::fs::File::open(filepath).unwrap();
        let filemap = unsafe { memmap2::Mmap::map(&file).unwrap() };
        Self::read_from_memmap(&filemap)
    }

    pub fn read_from_memmap(filemap: &memmap2::Mmap) -> Self {
        let identifier = Identifier::read_from_memmap(&filemap);
        let endianess = identifier.get_endianess();

        let mut offset: arch::Off = 16;

        Self {
            identifier,
            etype: arch::read_and_seek_half(filemap, &mut offset, &endianess),
            machine: arch::read_and_seek_half(filemap, &mut offset, &endianess),
            version: arch::read_and_seek_word(filemap, &mut offset, &endianess),
            entry: arch::read_and_seek_addr(filemap, &mut offset, &endianess),
            phoff: arch::read_and_seek_off(filemap, &mut offset, &endianess),
            shoff: arch::read_and_seek_off(filemap, &mut offset, &endianess),
            flags: arch::read_and_seek_word(filemap, &mut offset, &endianess),
            ehsize: arch::read_and_seek_half(filemap, &mut offset, &endianess),
            phentsize: arch::read_and_seek_half(filemap, &mut offset, &endianess),
            phnum: arch::read_and_seek_half(filemap, &mut offset, &endianess),
            shentsize: arch::read_and_seek_half(filemap, &mut offset, &endianess),
            shnum: arch::read_and_seek_half(filemap, &mut offset, &endianess),
            shstrndx: arch::read_and_seek_half(filemap, &mut offset, &endianess),
        }
    }

    pub fn read_nth_section_header(
        &self,
        filemap: &memmap2::Mmap,
        index: arch::Half,
    ) -> crate::SectionHeader {
        let mut offset = self.shoff + (index * self.shentsize) as arch::Off;
        let endianess = self.get_identifier().get_endianess();

        crate::SectionHeader::read_from_memmap(&filemap, &mut offset, &endianess)
    }

    pub fn read_nth_program_header(
        &self,
        filemap: &memmap2::Mmap,
        index: arch::Half,
    ) -> crate::ProgramHeader {
        let mut offset = self.phoff + (index * self.phentsize) as arch::Off;
        let endianess = self.get_identifier().get_endianess();

        crate::ProgramHeader::read_from_memmap(&filemap, &mut offset, &endianess)
    }

    pub fn get_etype(&self) -> Type {
        Type::from(self.etype)
    }

    pub fn get_etype_str(&self) -> String {
        self.get_etype().as_str().to_string()
    }

    pub fn get_machine(&self) -> Machine {
        Machine::from(self.machine)
    }

    pub fn get_machine_str(&self) -> &str {
        self.get_machine().as_str()
    }

    pub fn get_version(&self) -> Version {
        Version::from(self.version)
    }

    pub fn get_version_str(&self) -> &str {
        self.get_version().as_str()
    }

    pub fn get_entry_str(&self) -> String {
        format!("{:#x}", self.entry)
    }

    pub fn get_phoff_str(&self) -> String {
        format!("{}", self.phoff)
    }

    pub fn get_shoff_str(&self) -> String {
        format!("{}", self.shoff)
    }

    pub fn get_flags_str(&self) -> String {
        format!("0x{:x}", self.flags)
    }

    pub fn get_ehsize_str(&self) -> String {
        format!("{} bytes", self.ehsize)
    }

    pub fn get_phentsize_str(&self) -> String {
        format!("{} bytes", self.phentsize)
    }

    pub fn get_phnum_str(&self) -> String {
        self.phnum.to_string()
    }

    pub fn get_shentsize_str(&self) -> String {
        format!("{} bytes", self.shentsize)
    }

    pub fn get_shnum_str(&self) -> String {
        self.shnum.to_string()
    }

    pub fn get_shstrndx_str(&self) -> String {
        match self.shstrndx {
            0 => String::from("No section name string table"),
            0xFFFF => String::from(
                "Section name string table index is greater than or equal to SHN_LORESERVE",
            ),
            _ => self.shstrndx.to_string(),
        }
    }

    pub fn get_identifier(&self) -> Identifier {
        self.identifier.clone()
    }
}

impl core::fmt::Display for Header {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "\tELF Header {{
                Identifier: {},
                Type: {},
                Machine: {},
                Version: {},
                Entry point address: {},
                Program header offset: {},
                Section header offset: {},
                Flags: {},
                ELF header size: {},
                Program header entry size: {},
                Number of program headers: {},
                Section header entry size: {},
                Number of section headers: {},
                Section name string table index: {}
            }}",
            self.get_identifier(),
            self.get_etype_str(),
            self.get_machine_str(),
            self.get_version_str(),
            self.get_entry_str(),
            self.get_phoff_str(),
            self.get_shoff_str(),
            self.get_flags_str(),
            self.get_ehsize_str(),
            self.get_phentsize_str(),
            self.get_phnum_str(),
            self.get_shentsize_str(),
            self.get_shnum_str(),
            self.get_shstrndx_str()
        )
    }
}

impl core::fmt::Debug for Header {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "ELF Header {{
                Identifier: {},
                Type: {},
                Machine: {},
                Version: {},
                Entry point address: {},
                Program header offset: {},
                Section header offset: {},
                Flags: {},
                ELF header size: {},
                Program header entry size: {},
                Number of program headers: {},
                Section header entry size: {},
                Number of section headers: {},
                Section name string table index: {}
            }}",
            self.get_identifier(),
            self.get_etype_str(),
            self.get_machine_str(),
            self.get_version_str(),
            self.get_entry_str(),
            self.get_phoff_str(),
            self.get_shoff_str(),
            self.get_flags_str(),
            self.get_ehsize_str(),
            self.get_phentsize_str(),
            self.get_phnum_str(),
            self.get_shentsize_str(),
            self.get_shnum_str(),
            self.get_shstrndx_str()
        )
    }
}
