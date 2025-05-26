pub use memmap2;

pub mod arch32;
pub mod arch64;

pub use arch32::ELF as ELF32;
pub use arch64::ELF as ELF64;
