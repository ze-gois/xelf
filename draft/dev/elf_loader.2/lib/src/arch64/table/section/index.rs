pub enum Index {
    UNDEF = 0,
    // LORESERVE = 0xFF00,
    LOPROC = 0xFF00,
    HIPROC = 0xFF1F,
    ABS = 0xFFF1,
    COMMON = 0xFFF2,
    HIRESERVE = 0xFFFF,
}

impl Index {
    //pub fn from()
}

pub enum I2 {
    UNDEF = 0,       /* Used to mark an undefined or meaningless section reference */
    LOPROC = 0xFF00, /* Processor-specific use */
    HIPROC = 0xFF1F, /* */
    LOOS = 0xFF20,   /* Environment-specific use */
    HIOS = 0xFF3F,   /* */
    ABS = 0xFFF1,    /* Indicates that the corresponding reference is an absolute value */
    COMMON = 0xFFF2, /* Indicates a symbol that has been declared as a common block (Fortran COMMON or C tentative declaration) */
}
