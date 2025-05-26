//! As described here, the object file format supports various processors with 8-bit bytes and 64-bit
//! architectures. Nevertheless, it is intended to be extensible to larger (or smaller) architectures.
//! Object files therefore represent some control data with a machine-independent format, making
//! it possible to identify object files and interpret their contents in a common way. Remaining
//! data in an object file use the encoding of the target processor, regardless of the machine on
//! which the file was created.
//!
//! The data structures described in this document are described in a machine- independent format,
//! using symbolic data types.
//! The data structures are arranged so that fields are aligned on their natural
//! boundaries and the size of each structure is a multiple of the largest field in the
//! structure without padding.
//!
//! For 64-bit processors, these data types have the sizes and alignments shown.
//!
//! | Name         | Size | Alignment |         Purpose        |
//! |:------------:|:----:|:---------:|:----------------------:|
//! | Addr         |  8   |     8     |Unsigned program address|
//! | Off          |  8   |     8     |Unsigned file offset    |
//! | Half         |  2   |     2     |Unsigned medium integer |
//! | Word         |  4   |     4     |Unsigned integer        |
//! | SWord        |  4   |     4     |Signed integer          |
//! | XWord        |  8   |     8     |Unsigned long integer   |
//! | SXWord       |  8   |     8     |Signed long integer     |
//! | UnsignedChar |  1   |     1     |Unsigned small integer  |
//!
//! All data structures that the object file format defines follow the "natural'' size and alignment
//! guidelines for the relevant class. If necessary, data structures contain explicit padding to ensure
//! 4-byte alignment for 4-byte objects, to force structure sizes to a multiple of 4, and so on. Data
//! also have suitable alignment from the beginning of the file
pub mod dtype;
pub use dtype::*;

pub mod endianess;
pub use endianess::Endianess;

pub mod memory;

pub mod syscall;
