use super::super::dtype::*;

use crate::fs::DynFile;
use crate::Result;

use alloc::sync::Arc;

// pub const ArcClone: FnOnce(Arc<DynFile>) -> Arc<DynFile> = |df| Arc::clone(&df);

pub struct Header {
    pub p_type: Word,
    pub p_flags: Word,
    pub p_offset: Off,
    pub p_vaddr: Addr,
    pub p_paddr: Addr,
    pub p_filesz: XWord,
    pub p_memsz: XWord,
    pub p_align: XWord,
}

impl core::default::Default for Header {
    fn default() -> Self {
        Header {
            p_type: Word::default(),
            p_flags: Word::default(),
            p_offset: Off::default(),
            p_vaddr: Addr::default(),
            p_paddr: Addr::default(),
            p_filesz: XWord::default(),
            p_memsz: XWord::default(),
            p_align: XWord::default(),
        }
    }
}

impl Header {
    pub async fn from_file(
        offset: &mut u64,
        file: Arc<DynFile>,
        endianness: Endianness,
    ) -> Result<Self> {
        let mut header = Self::default();

        header.p_type = Word::read(offset, file.clone(), endianness).await?;
        header.p_flags = Word::read(offset, file.clone(), endianness).await?;
        header.p_offset = Off::read(offset, file.clone(), endianness).await?;
        header.p_vaddr = Addr::read(offset, file.clone(), endianness).await?;
        header.p_paddr = Addr::read(offset, file.clone(), endianness).await?;
        header.p_filesz = XWord::read(offset, file.clone(), endianness).await?;
        header.p_memsz = XWord::read(offset, file.clone(), endianness).await?;
        header.p_align = XWord::read(offset, file.clone(), endianness).await?;

        Ok(header)
    }
}
