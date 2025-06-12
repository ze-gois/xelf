pub use crate::dtype;

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
    pub etype: dtype::Half,
    /// # Machine type
    ///
    /// Target architecture defined in the processor-specific supplements.
    pub machine: dtype::Half,
    /// # Object file version
    ///
    /// Currently, this field has the value 1 (EV_CURRENT).
    pub version: dtype::Word,
    /// # Entry point address
    ///
    /// Virtual address of the program entry point.
    /// If there is no entry point, this field contains zero.
    pub entry: dtype::Addr,
    /// # Program header offset
    ///
    /// In bytes
    pub phoff: dtype::Off,
    /// # Section header offset
    ///
    /// The offset is expressed in bytes from the beginning of the ELF object file
    pub shoff: dtype::Off,
    /// # Processor-specific flags
    ///
    ///
    pub flags: dtype::Word,
    /// # ELF header size
    ///
    ///
    pub ehsize: dtype::Half,
    /// # Size of program header entry
    ///
    ///
    pub phentsize: dtype::Half,
    /// # Number of program header entries
    ///
    ///
    pub phnum: dtype::Half,
    /// # Size of section header entry
    ///
    ///
    pub shentsize: dtype::Half,
    /// # Number of section header entries
    ///
    ///
    pub shnum: dtype::Half,
    /// # Section name string table index
    ///
    /// Index whithin the Section Header Table.
    /// Despite being a dtype::Half it is used as usize.
    pub shstrndx: dtype::Half,
}

impl Header {
    pub fn from_filepath(filepath: &str) -> crate::Result<Self> {
        let file_descriptor = crate::open_filepath(filepath);
        Self::from_file_descriptor(&file_descriptor)
    }

    pub fn from_file_descriptor(file_descriptor: isize) -> crate::Result<Self> {
        let identifier = Identifier::from_file_descriptor(&file_descriptor);
        let endianness = identifier.get_endianness();

        // let mut offset: dtype::Off = 16;
        syscall::lseek(file_descriptor, 16, syscall::lseek::Flag::SET.into());

        Ok(Self {
            identifier,
            etype: dtype::Half::read(file_descriptor, endianness),
            machine: dtype::Half::read(file_descriptor, endianness),
            version: dtype::Word::read(file_descriptor, endianness),
            entry: dtype::Addr::read(file_descriptor, endianness),
            phoff: dtype::Off::read(file_descriptor, endianness),
            shoff: dtype::Off::read(file_descriptor, endianness),
            flags: dtype::Word::read(file_descriptor, endianness),
            ehsize: dtype::Half::read(file_descriptor, endianness),
            phentsize: dtype::Half::read(file_descriptor, endianness),
            phnum: dtype::Half::read(file_descriptor, endianness),
            shentsize: dtype::Half::read(file_descriptor, endianness),
            shnum: dtype::Half::read(file_descriptor, endianness),
            shstrndx: dtype::Half::read(file_descriptor, endianness),
        })
    }

    pub fn nth_section_header(
        &self,
        file_descriptor: isize,
        index: dtype::Half,
    ) -> crate::SectionHeader {
        let mut offset = self.shoff + (index * self.shentsize.0);
        let endianess = self.get_identifier().get_endianess();

        syscall::lseek(file_descriptor, offset, syscall::lseek::Flag::SET.into());

        crate::SectionHeader::from_file_descriptor(file_descriptor, &endianess)
    }

    pub fn nth_program_header(
        &self,
        file_descriptor: isize,
        index: dtype::Half,
    ) -> crate::ProgramHeader {
        let mut offset = self.phoff + (index * self.phentsize);

        syscall::lseek(file_descriptor, offset, syscall::lseek::Flag::SET.into());

        let endianess = self.get_identifier().get_endianess();

        crate::ProgramHeader::from_memmap(&file_descriptor, &endianess)
    }

    pub fn get_etype(&self) -> Type {
        Type::from(self.etype)
    }

    pub fn get_etype_str(&self) -> &str {
        self.get_etype().as_str()
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
