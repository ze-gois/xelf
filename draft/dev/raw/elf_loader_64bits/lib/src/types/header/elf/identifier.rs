use crate::types::data::arch64::UnsignedChar;

type IdentifierMemberType = UnsignedChar;

#[derive(Copy)]
pub struct Identifier {
    mag: [IdentifierMemberType; 4],
    class: IdentifierMemberType,
    data: IdentifierMemberType,
    version: IdentifierMemberType,
    osabi: IdentifierMemberType,
    abiversion: IdentifierMemberType,
    pad: IdentifierMemberType,
    un: [IdentifierMemberType; 5],
    nident: IdentifierMemberType,
}

impl Clone for Identifier {
    fn clone(&self) -> Self {
        Self {
            mag: self.mag.clone(),
            class: self.class.clone(),
            data: self.data.clone(),
            version: self.version.clone(),
            osabi: self.osabi.clone(),
            abiversion: self.abiversion.clone(),
            pad: self.pad.clone(),
            un: self.un.clone().clone(),
            nident: self.nident.clone(),
        }
    }
}

impl Identifier {
    pub fn load_from_memmap(fm: &memmap2::Mmap) -> Self {
        Self {
            mag: [fm[0], fm[1], fm[2], fm[3]],
            class: fm[4],
            data: fm[5],
            version: fm[6],
            osabi: fm[7],
            abiversion: fm[8],
            pad: fm[9],
            un: [fm[10], fm[11], fm[12], fm[13], fm[14]],
            nident: fm[15],
        }
    }

    pub fn load_from_filepath(fp: &str) -> Self {
        let f = std::fs::File::open(fp).unwrap();
        let fm = unsafe { memmap2::Mmap::map(&f).unwrap() };
        Identifier::load_from_memmap(&fm)
    }

    pub fn get_class(&self) -> IdentifierMemberType {
        self.class.clone()
    }

    pub fn get_class_str(&self) -> &str {
        match self.class {
            0 => "None",
            1 => "32-bit",
            2 => "64-bit",
            _ => "Invalid",
        }
    }

    pub fn get_data(&self) -> IdentifierMemberType {
        self.data.clone()
    }

    pub fn get_data_str(&self) -> &str {
        match self.data {
            0 => "None",
            1 => "LSB",
            2 => "MSB",
            _ => "Invalid",
        }
    }

    pub fn get_version(&self) -> IdentifierMemberType {
        self.version.clone()
    }

    pub fn get_osabi(&self) -> IdentifierMemberType {
        self.osabi.clone()
    }

    pub fn get_osabi_str(&self) -> &str {
        match self.osabi {
            0 => "SysV",
            1 => "HP-UX",
            255 => "Standalone",
            _ => "Invalid",
        }
    }

    pub fn get_abi_version(&self) -> IdentifierMemberType {
        self.abiversion.clone()
    }

    pub fn get_padding(&self) -> IdentifierMemberType {
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
            self.get_class(),
            self.get_class_str(),
            self.get_data(),
            self.get_data_str(),
            self.get_version(),
            self.get_osabi(),
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
        f.debug_struct("ELF Identifier")
            .field(
                "Class",
                &format!("{} ({})", self.get_class(), self.get_class_str()),
            )
            .field(
                "Data",
                &format!("{} ({})", self.get_data(), self.get_data_str()),
            )
            .field("Version", &self.get_version())
            .field(
                "OS ABI",
                &format!("{} ({})", self.get_osabi(), self.get_osabi_str()),
            )
            .field("ABI version", &self.get_abi_version())
            .field("Padding", &self.get_padding())
            .field("Unused", &self.get_unused_str())
            .field("NIdent", &self.nident)
            .finish()
    }
}
