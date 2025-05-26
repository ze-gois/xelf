/// EI_CLASS
///
/// The next byte, e_ident[EI_CLASS], identifies the file's class, or capacity.
///
/// The file format is designed to be portable among machines
/// of various sizes, without imposing the sizes of the largest
/// machine on the smallest. The class of the file defines the
/// basic types used by the data structures of the object file
/// container itself. The data contained in object file sections
/// may follow a different programming model. If so, the processor
/// supplement describes the model used.
///
/// Class ELFCLASS32 supports machines with 32-bit architectures.
/// It uses the basic types defined in the table labeled ``32-Bit Data Types.''
/// Class ELFCLASS64 supports machines with 64-bit architectures.
/// It uses the basic types defined in the table labeled ``64-Bit Data Types.''
/// Other classes will be defined as necessary, with different
/// basic types and sizes for object file data.
use crate::fs::DynFile;
use crate::{Error, Result};

use super::super::super::dtype::{Endianness, UChar};
use super::super::super::info;

pub const OFFSET: u64 = super::EI::CLASS as u64;

pub enum Class {
    ELFCLASSNONE = 0, // Invalid class
    ELFCLASS32 = 1,   // 32-bit objects
    ELFCLASS64 = 2,   // 64-bit objects
}

impl Class {
    pub fn from_u8(value: u8) -> Result<Self> {
        match value {
            0 => Ok(Class::ELFCLASSNONE),
            1 => Ok(Class::ELFCLASS32),
            2 => Ok(Class::ELFCLASS64),
            _ => Err(Error::InvalidParam),
        }
    }
}

pub async fn read(file: Arc<DynFile>) -> Result<Class> {
    let mut offset: u64 = OFFSET;

    let mut value = UChar::default();

    match UChar::read_buffer(&mut offset, file.clone(), Endianness::Little, &mut value).await {
        Ok(_) => {
            info!("ABI Version: {}\n", value.0);
            Class::from_u8(value.0)
        }
        Err(_) => {
            info!("Cannot read ABI Version\n");
            Err(Error::InvalidParam)
        }
    }
}
