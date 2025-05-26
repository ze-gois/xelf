/// This member identifies the object file type.
///
/// Although the core file contents are unspecified, type
/// ET_CORE is reserved to mark the file. Values from ET_LOOS
/// through ET_HIOS (inclusive) are reserved for operating
/// system-specific semantics. Values from ET_LOPROC through
/// ET_HIPROC (inclusive) are reserved for processor-specific
/// semantics. If meanings are specified, the processor supplement
/// explains them. Other values are reserved and will be assigned
/// to new object file types as necessary.

// No file type
pub const ET_NONE: u16 = 0;
// Relocatable file
pub const ET_REL: u16 = 1;
// Executable file
pub const ET_EXEC: u16 = 2;
// Shared object file
pub const ET_DYN: u16 = 3;
// Core file
pub const ET_CORE: u16 = 4;
// Operating system-specific
pub const ET_LOOS: u16 = 0xfe00;
// Operating system-specific
pub const ET_HIOS: u16 = 0xfeff;
// Processor-specific
pub const ET_LOPROC: u16 = 0xff00;
// Processor-specific
pub const ET_HIPROC: u16 = 0xffff;
