pub mod identifier;

pub use crate::types::data::arch64 as arch;

pub use identifier::Identifier;

// const NIDENT: usize = 16;

#[repr(C)]
pub struct Header64 {
    identifier: Identifier,
    etype: arch::Half,     /* Object file type */
    machine: arch::Half,   /* Machine type */
    version: arch::Word,   /* Object file version */
    entry: arch::Addr,     /* Entry point address */
    phoff: arch::Off,      /* Program header offset */
    shoff: arch::Off,      /* Section header offset */
    flags: arch::Word,     /* Processor-specific flags */
    ehsize: arch::Half,    /* ELF header size */
    phentsize: arch::Half, /* Size of program header entry */
    phnum: arch::Half,     /* Number of program header entries */
    shentsize: arch::Half, /* Size of section header entry */
    shnum: arch::Half,     /* Number of section header entries */
    shstrndx: arch::Half,  /* Section name string table index */
}

impl Header64 {
    pub fn load_from_filemap(filemap: &memmap2::Mmap) -> Result<Self, &str> {
        let identifier = Identifier::load(&filemap);

        return match identifier.get_data() {
            1 => Ok(Header64 {
                identifier,
                etype: arch::Half::from_le_bytes(filemap[16..18].try_into().unwrap()),
                machine: arch::Half::from_le_bytes(filemap[18..20].try_into().unwrap()),
                version: arch::Word::from_le_bytes(filemap[20..24].try_into().unwrap()),
                entry: arch::Addr::from_le_bytes(filemap[24..32].try_into().unwrap()),
                phoff: arch::Off::from_le_bytes(filemap[32..40].try_into().unwrap()),
                shoff: arch::Off::from_le_bytes(filemap[40..48].try_into().unwrap()),
                flags: arch::Word::from_le_bytes(filemap[48..52].try_into().unwrap()),
                ehsize: arch::Half::from_le_bytes(filemap[52..54].try_into().unwrap()),
                phentsize: arch::Half::from_le_bytes(filemap[54..56].try_into().unwrap()),
                phnum: arch::Half::from_le_bytes(filemap[56..58].try_into().unwrap()),
                shentsize: arch::Half::from_le_bytes(filemap[58..60].try_into().unwrap()),
                shnum: arch::Half::from_le_bytes(filemap[60..62].try_into().unwrap()),
                shstrndx: arch::Half::from_le_bytes(filemap[62..64].try_into().unwrap()),
            }),
            2 => Ok(Header64 {
                identifier,
                etype: arch::Half::from_be_bytes(filemap[16..18].try_into().unwrap()),
                machine: arch::Half::from_be_bytes(filemap[18..20].try_into().unwrap()),
                version: arch::Word::from_be_bytes(filemap[20..24].try_into().unwrap()),
                entry: arch::Addr::from_be_bytes(filemap[24..32].try_into().unwrap()),
                phoff: arch::Off::from_be_bytes(filemap[32..40].try_into().unwrap()),
                shoff: arch::Off::from_be_bytes(filemap[40..48].try_into().unwrap()),
                flags: arch::Word::from_be_bytes(filemap[48..52].try_into().unwrap()),
                ehsize: arch::Half::from_be_bytes(filemap[52..54].try_into().unwrap()),
                phentsize: arch::Half::from_be_bytes(filemap[54..56].try_into().unwrap()),
                phnum: arch::Half::from_be_bytes(filemap[56..58].try_into().unwrap()),
                shentsize: arch::Half::from_be_bytes(filemap[58..60].try_into().unwrap()),
                shnum: arch::Half::from_be_bytes(filemap[60..62].try_into().unwrap()),
                shstrndx: arch::Half::from_be_bytes(filemap[62..64].try_into().unwrap()),
            }),
            _ => Err("The data entry in ELF header identifier is not 0 (LSB) nor 1 (MSB)"),
        };
    }

    pub fn get_etype(&self) -> arch::Half {
        self.etype.clone()
    }

    pub fn get_etype_str(&self) -> String {
        match self.get_etype() {
            0 => String::from("NONE"),
            1 => String::from("REL"),
            2 => String::from("EXEC"),
            3 => String::from("DYN"),
            4 => String::from("CORE"),
            0xFE00..=0xFEFF => String::from("Environment-specific"),
            0xFF00..=0xFFFF => String::from("Processor-specific"),
            _ => format!("Unknown ({})", self.get_etype()),
        }
    }

    pub fn get_machine(&self) -> arch::Half {
        self.machine.clone()
    }

    pub fn get_machine_str(&self) -> String {
        match self.get_machine() {
            0 => String::from("No specific instruction set"),
            1 => String::from("AT&T WE 32100"),
            2 => String::from("SPARC"),
            3 => String::from("Intel x86"),
            4 => String::from("Motorola 68000 (M68k)"),
            5 => String::from("Motorola 88000 (M88k)"),
            7 => String::from("Intel 80860"),
            8 => String::from("MIPS"),
            62 => String::from("AMD x86-64"),
            _ => format!("Unknown ({})", self.get_machine()),
        }
    }

    pub fn get_version(&self) -> arch::Word {
        self.version.clone()
    }

    pub fn get_version_str(&self) -> String {
        match self.get_version() {
            1 => String::from("Current"),
            _ => format!("Unknown ({})", self.get_version()),
        }
    }

    pub fn get_entry(&self) -> arch::Addr {
        self.entry.clone()
    }

    pub fn get_entry_str(&self) -> String {
        format!("0x{:x}", self.get_entry())
    }

    pub fn get_phoff(&self) -> arch::Off {
        self.phoff.clone()
    }

    pub fn get_phoff_str(&self) -> String {
        format!("0x{:x}", self.get_phoff())
    }

    pub fn get_shoff(&self) -> arch::Off {
        self.shoff.clone()
    }

    pub fn get_shoff_str(&self) -> String {
        format!("0x{:x}", self.get_shoff())
    }

    pub fn get_flags(&self) -> arch::Word {
        self.flags.clone()
    }

    pub fn get_flags_str(&self) -> String {
        format!("0x{:x}", self.get_flags())
    }

    pub fn get_ehsize(&self) -> arch::Half {
        self.ehsize.clone()
    }

    pub fn get_ehsize_str(&self) -> String {
        format!("{} bytes", self.get_ehsize())
    }

    pub fn get_phentsize(&self) -> arch::Half {
        self.phentsize.clone()
    }

    pub fn get_phentsize_str(&self) -> String {
        format!("{} bytes", self.get_phentsize())
    }

    pub fn get_phnum(&self) -> arch::Half {
        self.phnum.clone()
    }

    pub fn get_phnum_str(&self) -> String {
        self.get_phnum().to_string()
    }

    pub fn get_shentsize(&self) -> arch::Half {
        self.shentsize.clone()
    }

    pub fn get_shentsize_str(&self) -> String {
        format!("{} bytes", self.get_shentsize())
    }

    pub fn get_shnum(&self) -> arch::Half {
        self.shnum.clone()
    }

    pub fn get_shnum_str(&self) -> String {
        self.get_shnum().to_string()
    }

    pub fn get_shstrndx(&self) -> arch::Half {
        self.shstrndx.clone()
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
impl std::fmt::Display for Header64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Header64 {{
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
