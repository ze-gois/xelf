use crate::arch;
use arch::Endianess;
use arch::UnsignedChar as T;

pub mod class;
pub use class::Class;

pub mod index;
pub use index::Index;

pub mod magic;
pub use magic::Magic;

pub mod os_abi;
pub use os_abi::OsAbi;

/// # Identifier to file as an ELF object file
///
/// Provide information about the data representation of the object file structures.
/// The bytes of this array that have defined meanings are detailed below.
/// The remaining bytes are reserved for future use, and should be set to zero.
#[derive(Copy)]
#[repr(C)]
pub struct Identifier {
    /// # Array with “magic numbers” identifying the file as an ELF object file.
    ///
    /// They contain the characters ‘\x7f’, ‘E’, ‘L’, and ‘F’, respectively.
    pub mag: [T; 4],
    /// # Identifies the class of the object file, or its capacity.
    ///
    /// |Name|Value|Meaning|
    /// |:-:|:-:|:-:|
    /// |C32|1|32-bit objects|
    /// |C64|2|64-bit objects|
    ///
    /// The class of the ELF file is independent of the data model assumed by the
    /// object code. The EI_CLASS field identifies the file format; a processor-
    /// specific flag in the e_flags field, described below, may be used to identify
    /// the application’s data model if the processory supports multiple models.
    pub class: T,
    /// # Data encoding of the object file data structures
    ///
    /// *endianess was originaly identified with "data"*
    ///
    /// |Name|Value|Meaning|
    /// |:-:|:-:|:-:|
    /// |None|0|Invalid endianness|
    /// |LSB|1|Little-endian|
    /// |MSB|2|Big-endian|
    ///
    /// For the convenience of code that examines ELF object files at run time
    /// (e.g., the dynamic loader), it is intended that the data encoding of the
    /// object file will match that of the running program. For environments that
    /// support both byte orders, a processor-specific flag in the e_flags field,
    /// described below, may be used to identify the application’s operating mode.
    pub endianess: T,
    /// # Version of the object file format.
    ///
    /// Currently, this field has the value
    /// CURRENT, which is defined with the value 1.
    pub version: T,
    /// # Operating system and ABI for which the object is prepared.
    ///
    /// Some fields in other ELF structures have flags and values
    /// that have environment-specific meanings; the interpretation of
    /// those fields is determined by the value of this field.
    pub osabi: T,
    /// # ABI version for which the object is prepared.
    ///
    /// Used to distinguish incompatible versions of an ABI.
    /// Interpretation of this version number is dependent on the ABI identified
    /// by the EI_OSABI field.
    /// For applications conforming to the System V ABI, third edition, this field
    /// should contain 0.
    pub abiversion: T,
    /// # Padding bytes
    pub padding: T,
    /// # Five unsassigned bytes
    pub unassigned: [T; 5],
    /// # Number of bytes of identifier
    pub nident: T,
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
            padding: self.padding.clone(),
            unassigned: self.unassigned.clone(),
            nident: self.nident.clone(),
        }
    }
}

impl Identifier {
    /// Loads ELF Identifier from a compliant path
    ///
    /// ```rust
    /// let path : &str = "./data/symver.powerpc64.so";
    /// let identifier = lib::header::elf::Identifier::read_from_filepath(&path);
    /// println!("{}",identifier);
    /// ```
    pub fn read_from_filepath<P: AsRef<std::path::Path>>(filepath: P) -> Self {
        let file = std::fs::File::open(filepath).unwrap();
        let filemap = unsafe { memmap2::Mmap::map(&file).unwrap() };
        Self::read_from_memmap(&filemap)
    }

    /// Loads ELF Identifier from a filemap
    ///
    /// ```rust
    /// let path : &str = "./data/symver.powerpc64.so";
    /// let file = core::fs::File::open(path).unwrap();
    /// let map = unsafe { memmap2::Mmap::map(&file).unwrap() };
    /// let identifier = lib::header::elf::Identifier::read_from_memmap(&map);
    /// println!("{}",identifier);
    /// ```
    pub fn read_from_memmap(fm: &memmap2::Mmap) -> Self {
        let mut offset = 0;
        let endianess = Endianess::LSB;

        Self {
            mag: [
                arch::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
                arch::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
                arch::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
                arch::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
            ],
            class: arch::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
            endianess: arch::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
            version: arch::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
            osabi: arch::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
            abiversion: arch::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
            padding: arch::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
            unassigned: [
                arch::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
                arch::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
                arch::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
                arch::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
                arch::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
            ],
            nident: arch::read_and_seek_unsigned_char(fm, &mut offset, &endianess),
        }
    }

    /// Each byte of the array is indexed symbolically using the names in the Table
    /// |Name|Value|Purpose|
    /// |:-:|:-:|:-:|
    /// |Mag0|0|File identification|
    /// |Mag1|1|File identification|
    /// |Mag2|2|File identification|
    /// |Mag3|3|File identification|
    /// |Class|4|File class|
    /// |Data|5|Data encoding|
    /// |Version|6|File version|
    /// |OsAbi|7|OS/ABI identification|
    /// |AbiVersion|8|ABI version|
    /// |Pad|9|Start of paddingdbytes|
    /// |Unassigned|10|Meaningless bytes|
    /// |Unassigned|11|Meaningless bytes|
    /// |Unassigned|12|Meaningless bytes|
    /// |Unassigned|13|Meaningless bytes|
    /// |Unassigned|14|Meaningless bytes|
    /// |Unassigned|15|Meaningless bytes|
    /// |NIdent|16|Size of e_ident[]|
    pub fn as_array(&self) -> [T; 16] {
        [
            self.mag[0],
            self.mag[1],
            self.mag[2],
            self.mag[3],
            self.class,
            self.endianess,
            self.version,
            self.osabi,
            self.abiversion,
            self.padding,
            self.unassigned[0],
            self.unassigned[1],
            self.unassigned[2],
            self.unassigned[3],
            self.unassigned[3],
            self.nident,
        ]
    }

    pub fn from_index(&self, i: Index) -> T {
        *self.as_array().get(i.to() as usize).unwrap()
    }

    pub fn from_usize(&self, i: usize) -> T {
        *self.as_array().get(i).unwrap()
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

    pub fn get_endianess_str(&self) -> &str {
        self.get_endianess().as_str()
    }

    pub fn copy_version(&self) -> T {
        self.version.clone()
    }

    pub fn copy_osabi(&self) -> OsAbi {
        OsAbi::from(self.osabi)
    }

    pub fn get_osabi_str(&self) -> &str {
        self.copy_osabi().as_str()
    }

    pub fn copy_abi_version(&self) -> T {
        self.abiversion.clone()
    }

    pub fn copy_padding(&self) -> T {
        self.padding.clone()
    }

    pub fn get_unassigned_str(&self) -> String {
        format!(
            "[{},{},{},{},{}]",
            self.unassigned[0],
            self.unassigned[1],
            self.unassigned[2],
            self.unassigned[3],
            self.unassigned[4]
        )
    }
}

impl core::fmt::Display for Identifier {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            formatter,
            "
            \tELF Identifier {{
                \tClass : {} ({})
                \tData : {} ({})
                \tVersion : {}
                \tOS ABI : {} ({})
                \tABI version : {}
                \tPadding : {}
                \tUnused : {}
                \tNIdent : {}
            \t}}
            ",
            self.class,
            self.get_class_str(),
            self.endianess,
            self.get_endianess_str(),
            self.copy_version(),
            self.osabi,
            self.get_osabi_str(),
            self.copy_abi_version(),
            self.copy_padding(),
            self.get_unassigned_str(),
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
                &format!("{} ({})", self.endianess, self.get_endianess_str()),
            )
            .field("Version", &self.copy_version())
            .field(
                "OS ABI",
                &format!("{} ({})", self.osabi, self.get_osabi_str()),
            )
            .field("ABI version", &self.copy_abi_version())
            .field("Padding", &self.copy_padding())
            .field("Unused", &self.get_unassigned_str())
            .field("NIdent", &self.nident)
            .finish()
    }
}
