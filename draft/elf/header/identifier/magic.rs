/// EI_MAG0 to EI_MAG3
///
/// A file's first 4 bytes hold a ``magic number,''
/// identifying the file as an ELF object file.
use crate::fs::DynFile;
use crate::{Error, Result};
use alloc::sync::Arc;
use alloc::vec::Vec;

use super::super::super::dtype::{Endianness, UChar};
use super::super::super::info;

pub const LENGTH: usize = 1 + (super::EI::MAG3 as usize) - (super::EI::MAG0 as usize);
pub const OFFSET: u64 = super::EI::DATA as u64;
pub const MAGIC: &[u8] = &[0x7f, 0x45, 0x4c, 0x46];

pub async fn read(file: Arc<DynFile>) -> Result<()> {
    let mut offset: u64 = OFFSET;
    let mut magic = Vec::<UChar>::with_capacity(LENGTH);

    return match UChar::read_vector(&mut offset, file, Endianness::Little, &mut magic).await {
        Ok(length) => {
            if length == LENGTH {
                is_valid(magic.as_slice())
            } else {
                info!("Cannot read ELF Magic on byte {:?}\n", length);
                Err(Error::InvalidParam)
            }
        }
        Err(_) => {
            info!("Cannot read ELF Magic\n");
            Err(Error::InvalidParam)
        }
    };
}

pub fn is_valid(magic: &[UChar]) -> Result<()> {
    if magic == MAGIC {
        return Ok(());
    }

    info!("Non-magical file\n");
    Err(Error::InvalidParam)
}
