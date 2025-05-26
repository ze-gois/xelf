/// EI_VERSION
///
/// This member identifies the object file version.
///
/// The value 1 signifies the original file format;
/// extensions will create new versions with higher numbers.
/// Although the value of EV_CURRENT is shown as 1 in the
/// previous table, it will change as necessary to reflect
/// the current version number.
use crate::fs::DynFile;
use crate::{Error, Result};

use super::super::super::dtype::{ELFType, Endianness, UChar};
use super::super::super::info;

use alloc::sync::Arc;

pub const OFFSET: u64 = super::EI::VERSION as u64;

pub enum Version {
    ELFVERSIONNONE = 0,    // Invalid version
    ELFVERSIONCURRENT = 1, // Current version
}

impl Version {
    pub fn from(value: <UChar as ELFType>::Inner) -> Result<Self> {
        match value {
            0 => Ok(Version::ELFVERSIONNONE),
            1 => Ok(Version::ELFVERSIONCURRENT),
            _ => Err(Error::InvalidParam),
        }
    }
}

pub async fn read(file: Arc<DynFile>) -> Result<Version> {
    let mut offset: u64 = OFFSET;
    let mut value = UChar::default();

    match UChar::read_buffer(&mut offset, file.clone(), Endianness::Little, &mut value).await {
        Ok(_) => {
            info!("ELF Version: {}\n", value.0);
            Version::from(value.0)
        }
        Err(_) => {
            info!("Cannot read ELF Version\n");
            Err(Error::InvalidParam)
        }
    }
}
