pub mod class;
pub mod endianess;
pub mod index;
pub mod magic;
pub mod os_abi;

pub use class::*;
pub use endianess::*;
pub use index::*;
pub use magic::*;
pub use os_abi::*;

pub use crate::arch32::data;

type T = data::UnsignedChar;

#[derive(Copy)]
pub struct Identifier {
    mag: [T; 4],
    class: T,
    endianess: T, // originally data
    version: T,
    osabi: T,
    abiversion: T,
    pad: T,
    un: [T; 5],
    nident: T,
}

impl Clone for Identifier {
    fn clone(&self) -> Self {
        Self {
            mag: self.mag.clone(),
            class: self.class.clone(),
            endianess: self.endianess.clone(),
            version: self.version.clone(),
            osabi: self.osabi.clone(),
            abiversion: self.abiversion.clone(),
            pad: self.pad.clone(),
            un: self.un.clone(),
            nident: self.nident.clone(),
        }
    }
}

impl Identifier {
    pub fn load_from_memmap(fm: &memmap2::Mmap) -> Self {
        let mut offset = 0;
        let endianess = Endianess::LSB;

        Self {
            mag: [
                data::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
                data::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
                data::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
                data::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
            ],
            class: data::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
            endianess: data::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
            version: data::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
            osabi: data::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
            abiversion: data::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
            pad: data::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
            un: [
                data::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
                data::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
                data::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
                data::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
                data::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
            ],
            nident: data::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
        }
    }

    pub fn load_from_filepath<P: AsRef<std::path::Path>>(filepath: P) -> Self {
        let file = std::fs::File::open(filepath).unwrap();
        let filemap = unsafe { memmap2::Mmap::map(&file).unwrap() };
        Self::load_from_memmap(&filemap)
    }

    pub fn get_class(&self) -> Class {
        Class::from(self.class)
    }

    pub fn get_class_str(&self) -> &str {
        self.get_class().as_str()
    }

    pub fn get_endianess(&self) -> Endianess {
        Endianess::from(self.endianess)
    }

    pub fn get_data_str(&self) -> &str {
        self.get_endianess().as_str()
    }

    pub fn get_version(&self) -> T {
        self.version.clone()
    }

    pub fn get_osabi(&self) -> OsAbi {
        OsAbi::from(self.osabi)
    }

    pub fn get_osabi_str(&self) -> &str {
        self.get_osabi().as_str()
    }

    pub fn get_abi_version(&self) -> T {
        self.abiversion.clone()
    }

    pub fn get_padding(&self) -> T {
        self.pad.clone()
    }

    pub fn get_unused_str(&self) -> String {
        format!(
            "[{},{},{},{},{}]",
            self.un[0].clone(),
            self.un[1].clone(),
            self.un[2].clone(),
            self.un[3].clone(),
            self.un[4].clone()
        )
    }
}

impl core::fmt::Display for Identifier {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            formatter,
            "
            ELF Identifier {{
                \tClass : {} ({})
                \tData : {} ({})
                \tVersion : {}
                \tOS ABI : {} ({})
                \tABI version : {}
                \tPadding : {}
                \tUnused : {}
                \tNIdent : {}
            }}
            ",
            self.class,
            self.get_class_str(),
            self.endianess,
            self.get_data_str(),
            self.get_version(),
            self.osabi,
            self.get_osabi_str(),
            self.get_abi_version(),
            self.get_padding(),
            self.get_unused_str(),
            self.nident,
        )
    }
}

impl core::fmt::Debug for Identifier {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ELF Header Identifier")
            .field(
                "Class",
                &format!("{} ({})", self.class, self.get_class_str()),
            )
            .field(
                "Data",
                &format!("{} ({})", self.endianess, self.get_data_str()),
            )
            .field("Version", &self.get_version())
            .field(
                "OS ABI",
                &format!("{} ({})", self.osabi, self.get_osabi_str()),
            )
            .field("ABI version", &self.get_abi_version())
            .field("Padding", &self.get_padding())
            .field("Unused", &self.get_unused_str())
            .field("NIdent", &self.nident)
            .finish()
    }
}
