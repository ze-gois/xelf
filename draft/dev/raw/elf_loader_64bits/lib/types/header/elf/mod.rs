pub mod identifier;

pub use crate::types::data::arch64::*;

use identifier::Identifier;

const NIDENT: usize = 16;

pub enum Elf64IType {
    NONE = 0,      /* No file type */
    REL = 1,       /* Relocatable object file */
    EXEC = 2,      /* Executable file */
    DYN = 3,       /* Shared object file */
    CORE = 4,      /* Core file */
    LOOS = 0xFE00, /* Environment-specific use */
    HIOS = 0xFEFF,
    LOPROC = 0xFF00, /* Processor-specific use */
    HIPROC = 0xFFFF,
}

pub struct Header64 {
    identifier: Identifier,
    etype: Elf64Half,     /* Object file type */
    machine: Elf64Half,   /* Machine type */
    version: Elf64Word,   /* Object file version */
    entry: Elf64Addr,     /* Entry point address */
    phoff: Elf64Off,      /* Program header offset */
    shoff: Elf64Off,      /* Section header offset */
    flags: Elf64Word,     /* Processor-specific flags */
    ehsize: Elf64Half,    /* ELF header size */
    phentsize: Elf64Half, /* Size of program header entry */
    phnum: Elf64Half,     /* Number of program header entries */
    shentsize: Elf64Half, /* Size of section header entry */
    shnum: Elf64Half,     /* Number of section header entries */
    shstrndx: Elf64Half,  /* Section name string table index */
}

impl Header64 {
    pub fn load_from_filepath(filepath: &str) -> Self {
        let elf_file = std::fs::File::open(filepath).unwrap();
        let elf_file_map = unsafe { memmap2::Mmap::map(&elf_file).unwrap() };
        let elf_header_identifier = Identifier::load(&elf_file_map);

        let header = Header64 {
            identifier: elf_header_identifier,
            etype: Elf64Half::from_le_bytes(elf_file_map[16..18].try_into().unwrap()),
            machine: Elf64Half::from_le_bytes(elf_file_map[18..20].try_into().unwrap()),
            version: Elf64Word::from_le_bytes(elf_file_map[20..24].try_into().unwrap()),
            entry: Elf64Addr::from_le_bytes(elf_file_map[24..32].try_into().unwrap()),
            phoff: Elf64Off::from_le_bytes(elf_file_map[32..40].try_into().unwrap()),
            shoff: Elf64Off::from_le_bytes(elf_file_map[40..48].try_into().unwrap()),
            flags: Elf64Word::from_le_bytes(elf_file_map[48..52].try_into().unwrap()),
            ehsize: Elf64Half::from_le_bytes(elf_file_map[52..54].try_into().unwrap()),
            phentsize: Elf64Half::from_le_bytes(elf_file_map[54..56].try_into().unwrap()),
            phnum: Elf64Half::from_le_bytes(elf_file_map[56..58].try_into().unwrap()),
            shentsize: Elf64Half::from_le_bytes(elf_file_map[58..60].try_into().unwrap()),
            shnum: Elf64Half::from_le_bytes(elf_file_map[60..62].try_into().unwrap()),
            shstrndx: Elf64Half::from_le_bytes(elf_file_map[62..64].try_into().unwrap()),
        };

        println!("{}", header);
        header
    }

    pub fn get_etype(&self) -> Elf64Half {
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

    pub fn get_machine(&self) -> Elf64Half {
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

    pub fn get_version(&self) -> Elf64Word {
        self.version.clone()
    }

    pub fn get_version_str(&self) -> String {
        match self.get_version() {
            1 => String::from("Current"),
            _ => format!("Unknown ({})", self.get_version()),
        }
    }

    pub fn get_entry(&self) -> Elf64Addr {
        self.entry.clone()
    }

    pub fn get_entry_str(&self) -> String {
        format!("0x{:x}", self.get_entry())
    }

    pub fn get_phoff(&self) -> Elf64Off {
        self.phoff.clone()
    }

    pub fn get_phoff_str(&self) -> String {
        format!("0x{:x}", self.get_phoff())
    }

    pub fn get_shoff(&self) -> Elf64Off {
        self.shoff.clone()
    }

    pub fn get_shoff_str(&self) -> String {
        format!("0x{:x}", self.get_shoff())
    }

    pub fn get_flags(&self) -> Elf64Word {
        self.flags.clone()
    }

    pub fn get_flags_str(&self) -> String {
        format!("0x{:x}", self.get_flags())
    }

    pub fn get_ehsize(&self) -> Elf64Half {
        self.ehsize.clone()
    }

    pub fn get_ehsize_str(&self) -> String {
        format!("{} bytes", self.get_ehsize())
    }

    pub fn get_phentsize(&self) -> Elf64Half {
        self.phentsize.clone()
    }

    pub fn get_phentsize_str(&self) -> String {
        format!("{} bytes", self.get_phentsize())
    }

    pub fn get_phnum(&self) -> Elf64Half {
        self.phnum.clone()
    }

    pub fn get_phnum_str(&self) -> String {
        self.get_phnum().to_string()
    }

    pub fn get_shentsize(&self) -> Elf64Half {
        self.shentsize.clone()
    }

    pub fn get_shentsize_str(&self) -> String {
        format!("{} bytes", self.get_shentsize())
    }

    pub fn get_shnum(&self) -> Elf64Half {
        self.shnum.clone()
    }

    pub fn get_shnum_str(&self) -> String {
        self.get_shnum().to_string()
    }

    pub fn get_shstrndx(&self) -> Elf64Half {
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
impl core::fmt::Display for Header64 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
