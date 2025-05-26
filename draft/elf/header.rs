pub mod etype;
/// Some object file control structures can grow, because
/// the ELF header contains their actual sizes. If the object
/// file format changes, a program may encounter control
/// structures that are larger or smaller than expected.
/// Programs might therefore ignore ``extra'' information.
/// The treatment of ``missing'' information depends on context
/// and will be specified when and if extensions are defined.
pub mod identifier;
pub mod machine;
pub mod version;

use super::dtype::*;
pub use identifier::Identifier;

use crate::fs::DynFile;
use crate::{Error, Result};

use alloc::sync::Arc;

pub struct Header {
    pub e_ident: Identifier,
    pub e_type: Half,
    pub e_machine: Half,
    pub e_version: Word,
    pub e_entry: Addr,
    pub e_phoff: Off,
    pub e_shoff: Off,
    pub e_flags: Word,
    pub e_ehsize: Half,
    pub e_phentsize: Half,
    pub e_phnum: Half,
    pub e_shentsize: Half,
    pub e_shnum: Half,
    pub e_shstrndx: Half,
}

impl Header {
    pub async fn read_file(file: Arc<DynFile>) -> Result<Self> {
        let e_ident = Identifier::from_file(file.clone()).await?;

        let endianness = e_ident.endianness()?;

        let mut offset: u64 = identifier::EI_NIDENT as u64;

        Ok(Header {
            e_ident,
            e_type: Half::read(&mut offset, file.clone(), endianness).await?,
            e_machine: Half::read(&mut offset, file.clone(), endianness).await?,
            e_version: Word::read(&mut offset, file.clone(), endianness).await?,
            e_entry: Addr::read(&mut offset, file.clone(), endianness).await?,
            e_phoff: Off::read(&mut offset, file.clone(), endianness).await?,
            e_shoff: Off::read(&mut offset, file.clone(), endianness).await?,
            e_flags: Word::read(&mut offset, file.clone(), endianness).await?,
            e_ehsize: Half::read(&mut offset, file.clone(), endianness).await?,
            e_phentsize: Half::read(&mut offset, file.clone(), endianness).await?,
            e_phnum: Half::read(&mut offset, file.clone(), endianness).await?,
            e_shentsize: Half::read(&mut offset, file.clone(), endianness).await?,
            e_shnum: Half::read(&mut offset, file.clone(), endianness).await?,
            e_shstrndx: Half::read(&mut offset, file.clone(), endianness).await?,
        })
    }

    pub fn endianness(&self) -> Result<Endianness> {
        Ok(match self.e_ident.data {
            identifier::Data::ELFDATA2LSB => Endianness::Little,
            identifier::Data::ELFDATA2MSB => Endianness::Big,
            identifier::Data::ELFDATANONE => return Err(Error::InvalidParam),
        })
    }

    pub fn is_dynamically_linked(&self) -> bool {
        self.e_type.0 == etype::ET_DYN
    }
}
