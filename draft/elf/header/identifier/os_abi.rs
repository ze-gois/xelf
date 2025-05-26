/// EI_OSABI
///
/// Byte e_ident[EI_OSABI] identifies the OS- or ABI-specific
/// ELF extensions used by this file. Some fields in other ELF
/// structures have flags and values that have operating system
/// and/or ABI specific meanings; the interpretation of those
/// fields is determined by the value of this byte. If the object
/// file does not use any extensions, it is recommended that this
/// byte be set to 0. If the value for this byte is 64 through 255,
/// its meaning depends on the value of the e_machine header member.
///
/// The ABI processor supplement for an architecture can define its
/// own associated set of values for this byte in this range. If the
/// processor supplement does not specify a set of values, one of the
/// following values shall be used, where 0 can also be taken to mean
/// unspecified.
use crate::fs::DynFile;
use crate::{Error, Result};

use super::super::super::dtype::{ELFType, Endianness, UChar};
use super::super::super::info;

use alloc::sync::Arc;

pub const OFFSET: u64 = super::EI::OSABI as u64;

#[allow(non_camel_case_types)]
pub enum OSABI {
    ELFOSABI_NONE = 0,           // No extensions or unspecified
    ELFOSABI_HPUX = 1,           // Hewlett-Packard HP-UX
    ELFOSABI_NETBSD = 2,         // NetBSD
    ELFOSABI_LINUX = 3,          // Linux
    ELFOSABI_SOLARIS = 6,        // Sun Solaris
    ELFOSABI_AIX = 7,            // AIX
    ELFOSABI_IRIX = 8,           // IRIX
    ELFOSABI_FREEBSD = 9,        // FreeBSD
    ELFOSABI_TRU64 = 10,         // Compaq TRU64 UNIX
    ELFOSABI_MODESTO = 11,       // Novell Modesto
    ELFOSABI_OPENBSD = 12,       // Open BSD
    ELFOSABI_OPENVMS = 13,       // Open VMS
    ELFOSABI_NSK = 14,           // Hewlett-Packard Non-Stop Kernel
    ELFOSABI_ARCHSPECIFIC = 255, // Architecture specific
}

impl OSABI {
    pub fn from(value: <UChar as ELFType>::Inner) -> Result<Self> {
        match value {
            1 => Ok(OSABI::ELFOSABI_HPUX),
            2 => Ok(OSABI::ELFOSABI_NETBSD),
            3 => Ok(OSABI::ELFOSABI_LINUX),
            6 => Ok(OSABI::ELFOSABI_SOLARIS),
            7 => Ok(OSABI::ELFOSABI_AIX),
            8 => Ok(OSABI::ELFOSABI_IRIX),
            9 => Ok(OSABI::ELFOSABI_FREEBSD),
            10 => Ok(OSABI::ELFOSABI_TRU64),
            11 => Ok(OSABI::ELFOSABI_MODESTO),
            12 => Ok(OSABI::ELFOSABI_OPENBSD),
            13 => Ok(OSABI::ELFOSABI_OPENVMS),
            14 => Ok(OSABI::ELFOSABI_NSK),
            64..=255 => Ok(OSABI::ELFOSABI_ARCHSPECIFIC),
            _ => Ok(OSABI::ELFOSABI_NONE),
        }
    }
}

pub async fn read(file: Arc<DynFile>) -> Result<OSABI> {
    let mut offset: u64 = OFFSET;
    let mut value = UChar::default();

    match UChar::read_buffer(&mut offset, file.clone(), Endianness::Little, &mut value).await {
        Ok(_) => {
            info!("OSABI Encoding: {}\n", value.0);
            OSABI::from(value.0)
        }
        Err(_) => {
            info!("Cannot read Data Encoding\n");
            Err(Error::InvalidParam)
        }
    }
}
