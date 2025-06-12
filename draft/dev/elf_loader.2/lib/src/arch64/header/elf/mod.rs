pub mod etype;
pub mod identifier;
pub mod machine;
pub mod version;

pub use etype::ElfType;
pub use identifier::Identifier;
pub use machine::Machine;
pub use version::Version;

pub use crate::arch64::data;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Header {
    identifier: Identifier,
    etype: data::Half,     /* Object file type */
    machine: data::Half,   /* Machine type */
    version: data::Word,   /* Object file version */
    entry: data::Addr,     /* Entry point address */
    phoff: data::Off,      /* Program header offset */
    shoff: data::Off,      /* Section header offset */
    flags: data::Word,     /* Processor-specific flags */
    ehsize: data::Half,    /* ELF header size */
    phentsize: data::Half, /* Size of program header entry */
    phnum: data::Half,     /* Number of program header entries */
    shentsize: data::Half, /* Size of section header entry */
    shnum: data::Half,     /* Number of section header entries */
    shstrndx: data::Half,  /* Section name string table index */
}

impl Header {
    pub fn load_from_filepath<P: AsRef<std::path::Path>>(filepath: P) -> Self {
        let file = std::fs::File::open(filepath).unwrap();
        let filemap = unsafe { memmap2::Mmap::map(&file).unwrap() };
        Self::load_from_memmap(&filemap)
    }

    pub fn load_from_memmap(filemap: &memmap2::Mmap) -> Self {
        let identifier = Identifier::load_from_memmap(&filemap);
        let endianess = identifier.get_endianess();

        let mut offset: data::Off = 16;

        Self {
            identifier,
            etype: data::read_and_seek_half(filemap, &mut offset, &endianess),
            machine: data::read_and_seek_half(filemap, &mut offset, &endianess),
            version: data::read_and_seek_word(filemap, &mut offset, &endianess),
            entry: data::read_and_seek_addr(filemap, &mut offset, &endianess),
            phoff: data::read_and_seek_off(filemap, &mut offset, &endianess),
            shoff: data::read_and_seek_off(filemap, &mut offset, &endianess),
            flags: data::read_and_seek_word(filemap, &mut offset, &endianess),
            ehsize: data::read_and_seek_half(filemap, &mut offset, &endianess),
            phentsize: data::read_and_seek_half(filemap, &mut offset, &endianess),
            phnum: data::read_and_seek_half(filemap, &mut offset, &endianess),
            shentsize: data::read_and_seek_half(filemap, &mut offset, &endianess),
            shnum: data::read_and_seek_half(filemap, &mut offset, &endianess),
            shstrndx: data::read_and_seek_half(filemap, &mut offset, &endianess),
        }
    }

    pub fn get_etype(&self) -> ElfType {
        ElfType::from(self.etype)
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

    pub fn get_entry(&self) -> data::Addr {
        self.entry
    }

    pub fn get_entry_str(&self) -> String {
        format!("{:#x}", self.get_entry())
    }

    pub fn get_phoff(&self) -> data::Off {
        self.phoff
    }

    pub fn get_phoff_str(&self) -> String {
        format!("{}", self.get_phoff())
    }

    pub fn get_shoff(&self) -> data::Off {
        self.shoff
    }

    pub fn get_shoff_str(&self) -> String {
        format!("{}", self.get_shoff())
    }

    pub fn get_flags(&self) -> data::Word {
        self.flags
    }

    pub fn get_flags_str(&self) -> String {
        format!("0x{:x}", self.get_flags())
    }

    pub fn get_ehsize(&self) -> data::Half {
        self.ehsize
    }

    pub fn get_ehsize_str(&self) -> String {
        format!("{} bytes", self.get_ehsize())
    }

    pub fn get_phentsize(&self) -> data::Half {
        self.phentsize
    }

    pub fn get_phentsize_str(&self) -> String {
        format!("{} bytes", self.get_phentsize())
    }

    pub fn get_phnum(&self) -> data::Half {
        self.phnum
    }

    pub fn get_phnum_str(&self) -> String {
        self.get_phnum().to_string()
    }

    pub fn get_shentsize(&self) -> data::Half {
        self.shentsize
    }

    pub fn get_shentsize_str(&self) -> String {
        format!("{} bytes", self.get_shentsize())
    }

    pub fn get_shnum(&self) -> data::Half {
        self.shnum
    }

    pub fn get_shnum_str(&self) -> String {
        self.get_shnum().to_string()
    }

    pub fn get_shstrndx(&self) -> data::Half {
        self.shstrndx
    }

    pub fn get_shstrndx_str(&self) -> String {
        match self.get_shstrndx() {
            0 => String::from("No section name string table"),
            0xFFFF => String::from(
                "Section name string table index is greater than or equal to SHN_LORESERVE",
            ),
            _ => self.get_shstrndx().to_string(),
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
            "Header64 {{
                \tIdentifier: {},
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
