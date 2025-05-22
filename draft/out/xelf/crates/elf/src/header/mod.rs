pub mod class;
pub mod etype;

pub use crate::dtype::*;
pub use crate::error::*;

pub use x86_64::syscall::write;

const ELF_MAGIC: [u8; 4] = [0x7f, b'E', b'L', b'F'];

#[repr(C)]
pub struct Header {
    pub ident: [UChar; 16], /* ELF identification */
    pub etype: Half,        /* Object file type */
    pub machine: Half,      /* Machine type */
    pub version: Word,      /* Object file version */
    pub entry: Addr,        /* Entry point address */
    pub phoff: Off,         /* Program header offset in file */
    pub shoff: Off,         /* Section header offset in file */
    pub flags: Word,        /* Processor-specific flags */
    pub ehsize: Half,       /* ELF header size */
    pub phentsize: Half,    /* Size of program header entry */
    pub phnum: Half,        /* Number of program header entries */
    pub shentsize: Half,    /* Size of section header entry */
    pub shnum: Half,        /* Number of section header entries */
    pub shstrndx: Half,     /* Section name string table index */
}

impl Header {
    pub unsafe fn read(fd: i32) -> Result<Self> {
        set_endianness(Endianness::Little);

        let elf_magic = [
            UChar::read(fd)?,
            UChar::read(fd)?,
            UChar::read(fd)?,
            UChar::read(fd)?,
        ];

        if elf_magic != ELF_MAGIC {
            return Err(Error::InvalidData);
        }

        let ident = [
            elf_magic[0],
            elf_magic[1],
            elf_magic[2],
            elf_magic[3],
            UChar::read(fd)?,
            UChar::read(fd)?,
            UChar::read(fd)?,
            UChar::read(fd)?,
            UChar::read(fd)?,
            UChar::read(fd)?,
            UChar::read(fd)?,
            UChar::read(fd)?,
            UChar::read(fd)?,
            UChar::read(fd)?,
            UChar::read(fd)?,
            UChar::read(fd)?,
        ];

        match ident[5].0 {
            1 => set_endianness(Endianness::Little),
            2 => set_endianness(Endianness::Big),
            _ => return Err(Error::InvalidData),
        }

        let etype = Half::read(fd)?;

        crate::set_dynamic(etype.0 == 3);

        Ok(Header {
            ident,
            etype,
            machine: Half::read(fd)?,
            version: Word::read(fd)?,
            entry: Addr::read(fd)?,
            phoff: Off::read(fd)?,
            shoff: Off::read(fd)?,
            flags: Word::read(fd)?,
            ehsize: Half::read(fd)?,
            phentsize: Half::read(fd)?,
            phnum: Half::read(fd)?,
            shentsize: Half::read(fd)?,
            shnum: Half::read(fd)?,
            shstrndx: Half::read(fd)?,
        })
    }

    pub unsafe fn print(&self) {
        x86_64::print_str("ELF Header {{");
        x86_64::print_str("\n\t[etype]: ");
        self.etype.print();
        x86_64::print_str(" (");
        self.etype.print_hex();
        x86_64::print_str(")");
        x86_64::print_str("\n\t[machine]: ");
        self.machine.print();
        x86_64::print_str(" (");
        self.machine.print_hex();
        x86_64::print_str(")");
        x86_64::print_str("\n\t[version]: ");
        self.version.print();
        x86_64::print_str(" (");
        self.version.print_hex();
        x86_64::print_str(")");
        x86_64::print_str("\n\t[entry]: ");
        self.entry.print();
        x86_64::print_str(" (");
        self.entry.print_hex();
        x86_64::print_str(")");
        x86_64::print_str("\n\t[phoff]: ");
        self.phoff.print();
        x86_64::print_str(" (");
        self.phoff.print_hex();
        x86_64::print_str(")");
        x86_64::print_str("\n\t[shoff]: ");
        self.shoff.print();
        x86_64::print_str(" (");
        self.shoff.print_hex();
        x86_64::print_str(")");
        x86_64::print_str("\n\t[flags]: ");
        self.flags.print();
        x86_64::print_str(" (");
        self.flags.print_hex();
        x86_64::print_str(")");
        x86_64::print_str("\n\t[ehsize]: ");
        self.ehsize.print();
        x86_64::print_str(" (");
        self.ehsize.print_hex();
        x86_64::print_str(")");
        x86_64::print_str("\n\t[phentsize]: ");
        self.phentsize.print();
        x86_64::print_str(" (");
        self.phentsize.print_hex();
        x86_64::print_str(")");
        x86_64::print_str("\n\t[phnum]: ");
        self.phnum.print();
        x86_64::print_str(" (");
        self.phnum.print_hex();
        x86_64::print_str(")");
        x86_64::print_str("\n\t[shentsize]: ");
        self.shentsize.print();
        x86_64::print_str(" (");
        self.shentsize.print_hex();
        x86_64::print_str(")");
        x86_64::print_str("\n\t[shnum]: ");
        self.shnum.print();
        x86_64::print_str(" (");
        self.shnum.print_hex();
        x86_64::print_str(")");
        x86_64::print_str("\n\t[shstrndx]: ");
        self.shstrndx.print();
        x86_64::print_str(" (");
        self.shstrndx.print_hex();
        x86_64::print_str(")");
        x86_64::print_str("\n} ELF Header");
    }

    // Add methods to verify other header fields
    pub fn is_valid(&self) -> bool {
        // Check class, data encoding, version, etc.
        true // TODO: implement actual checks
    }
}
