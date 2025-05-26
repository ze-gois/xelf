pub enum IType {
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
