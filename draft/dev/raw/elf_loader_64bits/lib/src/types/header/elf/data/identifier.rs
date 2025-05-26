pub enum Elf64Identifier {
    MAG0 = 0,       /* File identification */
    MAG1 = 1,       /* File identification */
    MAG2 = 2,       /* File identification */
    MAG3 = 3,       /* File identification */
    CLASS = 4,      /* File class */
    DATA = 5,       /* Data encoding */
    VERSION = 6,    /* File version  */
    OSABI = 7,      /* OS/ABI identification */
    ABIVERSION = 8, /* ABI version */
    PAD = 9,        /* Start of padding bytes */
    UN1 = 10,       /* Unassigned */
    UN2 = 11,       /* Unassigned */
    UN3 = 12,       /* Unassigned */
    UN4 = 13,       /* Unassigned */
    UN5 = 14,       /* Unassigned */
    UN6 = 15,       /* Unassigned */
    NIDENT = 16,    /* Size of ELF identifier array */
}

pub enum Elf64IMag {
    M0 = '\x7f',
    M1 = 'E',
    M2 = 'L',
    M3 = 'F',
}

pub enum Elf64IClass {
    C32 = 1, /* 32-bit objects */
    C64 = 2, /* 64-bit objects */
}

pub enum Elf64IData {
    LSB = 1, /* Object file data structures are little-
             endian */
    MSB = 2, /* Object file data structures are big-
             endian */
}

pub enum Elf64IOSABI {
    SYSV = 0, /* System V ABI */
    HPUX = 1, /* HP-UX operating system */
    STANDALNOE = 255, /* Standalone (embedded)
              application */
}
