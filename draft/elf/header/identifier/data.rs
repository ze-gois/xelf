/// EI_DATA
///
/// Byte e_ident[EI_DATA] specifies the encoding of both the data
/// structures used by object file container and data contained in
/// object file sections. The following encodings are currently defined.
///
/// Other values are reserved and will be assigned to new encodings as necessary.
///
/// NOTE: Primarily for the convenience of code that looks at the ELF file at
/// runtime, the ELF data structures are intended to have the same byte order
/// as that of the running program.
use crate::fs::DynFile;
use crate::{Error, Result};

use super::super::super::dtype::{ELFType, Endianness, UChar};
use super::super::super::info;

use alloc::sync::Arc;

pub const OFFSET: u64 = super::EI::DATA as u64;

pub enum Data {
    ELFDATANONE = 0, // Invalid data encoding
    ELFDATA2LSB = 1, // See below
    ELFDATA2MSB = 2, // See below
}

impl Data {
    pub fn from(value: <UChar as ELFType>::Inner) -> Result<Self> {
        match value {
            0 => Ok(Data::ELFDATANONE),
            1 => Ok(Data::ELFDATA2LSB),
            2 => Ok(Data::ELFDATA2MSB),
            _ => Err(Error::InvalidParam),
        }
    }
}

pub async fn read(file: Arc<DynFile>) -> Result<Data> {
    let mut offset: u64 = OFFSET;
    let mut value = UChar::default();

    match UChar::read_buffer(&mut offset, file.clone(), Endianness::Little, &mut value).await {
        Ok(_) => {
            info!("Data Encoding: {}\n", value.0);
            Data::from(value.0)
        }
        Err(_) => {
            info!("Cannot read Data Encoding\n");
            Err(Error::InvalidParam)
        }
    }
}
