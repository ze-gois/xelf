/// EI_ABIVERSION
///
/// Byte e_ident[EI_ABIVERSION] identifies the version of the
/// ABI to which the object is targeted. This field is used
/// to distinguish among incompatible versions of an ABI.
///
/// The interpretation of this version number is dependent on
/// the ABI identified by the EI_OSABI field. If no values are
/// specified for the EI_OSABI field by the processor supplement
/// or no version values are specified for the ABI determined by
/// a particular value of the EI_OSABI byte, the value 0 shall be
/// used for the EI_ABIVERSION byte; it indicates unspecified.
use crate::fs::DynFile;
use crate::{Error, Result};

use super::super::super::dtype::{ELFType, Endianness, UChar};
use super::super::super::info;

use alloc::sync::Arc;

pub const OFFSET: u64 = super::EI::DATA as u64;

pub async fn read(file: Arc<DynFile>) -> Result<<UChar as ELFType>::Inner> {
    let mut offset: u64 = OFFSET;

    let mut value = UChar::default();

    match UChar::read_buffer(&mut offset, file.clone(), Endianness::Little, &mut value).await {
        Ok(_) => {
            info!("ABI Version: {}\n", value.0);
            Ok(value.0)
        }
        Err(_) => {
            info!("Cannot read ABI Version\n");
            Err(Error::InvalidParam)
        }
    }
}
