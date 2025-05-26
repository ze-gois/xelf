pub mod abi_version;
pub mod class;
pub mod data;
pub mod magic;
pub mod os_abi;
pub mod version;

pub use class::*;
pub use data::*;
pub use os_abi::*;
pub use version::*;

use super::super::dtype::ELFType;
use super::super::dtype::Endianness;
use super::super::dtype::UChar;

use crate::fs::DynFile;
use crate::{Error, Result};

use alloc::sync::Arc;

// Size of e_ident[]
pub const EI_NIDENT: usize = 16;

enum EI {
    MAG0 = 0,       // File identification
    MAG1 = 1,       // File identification
    MAG2 = 2,       // File identification
    MAG3 = 3,       // File identification
    CLASS = 4,      // File class
    DATA = 5,       // Data encoding
    VERSION = 6,    // File version
    OSABI = 7,      // Operating system/ABI identification
    ABIVERSION = 8, // ABI version
    PAD = 9,        // Start of padding bytes
}

pub struct Identifier {
    pub class: class::Class,
    pub data: data::Data,
    pub version: version::Version,
    pub os_abi: os_abi::OSABI,
    pub abi_version: <UChar as ELFType>::Inner,
}

impl Identifier {
    pub async fn from_file(file: Arc<DynFile>) -> Result<Self> {
        magic::read(file.clone()).await?;

        Ok(Identifier {
            class: class::read(file.clone()).await?,
            data: data::read(file.clone()).await?,
            version: version::read(file.clone()).await?,
            os_abi: os_abi::read(file.clone()).await?,
            abi_version: abi_version::read(file.clone()).await?,
        })
    }

    pub fn endianness(&self) -> Result<Endianness> {
        Ok(match self.data {
            Data::ELFDATA2LSB => Endianness::Little,
            Data::ELFDATA2MSB => Endianness::Big,
            Data::ELFDATANONE => return Err(Error::InvalidParam),
        })
    }
}
