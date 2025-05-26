pub mod data;
pub mod identifier;

pub use identifier::Identifier;

pub use crate::types::data::arch32::*;
pub use crate::types::data::arch64::*;

// use crate::types::data::arch32::*;
// use crate::types::data::arch64::*;

// use crate::types::data::arch32::Elf32Half;

// use crate::types::header::data::*;

pub struct Header64 {
    pub identifier: Identifier,
    pub elf_type: Elf32Type, /* Object file type */
    // pub elf_type: Elf32Half,  /* Object file type */
    pub machine: Elf32Half,   /* Machine type */
    pub version: Elf32Word,   /* Object file version */
    pub entry: Elf32Addr,     /* Entry point address */
    pub phoff: Elf32Off,      /* Program header offset */
    pub shoff: Elf32Off,      /* Section header offset */
    pub flags: Elf32Word,     /* Processor-specific flags */
    pub ehsize: Elf32Half,    /* ELF header size */
    pub phentsize: Elf32Half, /* Size of program header entry */
    pub phnum: Elf32Half,     /* Number of program header entries */
    pub shentsize: Elf32Half, /* Size of section header entry */
    pub shnum: Elf32Half,     /* Number of section header entries */
    pub shstrndx: Elf32Half,  /* Section name string table index */
}

pub struct Header32 {
    pub identifier: Identifier,
    pub elf_type: Elf64Type, /* Object file type */
    // pub elf_type: Elf64Half,  /* Object file type */
    pub machine: Elf64Half,   /* Machine type */
    pub version: Elf64Word,   /* Object file version */
    pub entry: Elf64Addr,     /* Entry point address */
    pub phoff: Elf64Off,      /* Program header offset */
    pub shoff: Elf64Off,      /* Section header offset */
    pub flags: Elf64Word,     /* Processor-specific flags */
    pub ehsize: Elf64Half,    /* ELF header size */
    pub phentsize: Elf64Half, /* Size of program header entry */
    pub phnum: Elf64Half,     /* Number of program header entries */
    pub shentsize: Elf64Half, /* Size of section header entry */
    pub shnum: Elf64Half,     /* Number of section header entries */
    pub shstrndx: Elf64Half,  /* Section name string table index */
}
