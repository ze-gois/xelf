use crate::dtype;

pub mod entry;
pub use entry::Entry;
use syscall::lseek;

#[repr(C)]
#[derive(Clone)]
pub struct Table<'a> {
    pub offset: dtype::Off,
    pub entries: *mut Entry<'a>,
    pub counter: usize,
}

impl<'a> Table<'a> {
    pub fn read_from_section_header(
        file_descriptor: isize,
        string_table_entry: &crate::SectionEntry,
        endianness: dtype::Endianness,
    ) -> Self {
        let offset = string_table_entry.header.offset.0;
        syscall::lseek(file_descriptor as i32, offset as i64, lseek::Flag::SET.to());

        let size = string_table_entry.header.size.0 as usize;

        let entry_pointer = crate::alloc::<u8>(size).unwrap();

        for s in 0..size {
            unsafe {
                *entry_pointer.add(s) = dtype::UChar::read(file_descriptor, endianness).unwrap().0
            }
        }

        let entries_bytes = unsafe { core::slice::from_raw_parts_mut(entry_pointer, size) };
        let entries_split = entries_bytes.split(|byte| *byte == 0);

        let mut counter = 0;
        entries_split.for_each(|_| {
            counter = counter + 1;
        });

        let entries = crate::alloc::<Entry>(counter).unwrap();

        let mut count = 0;
        let mut offset_g: usize = string_table_entry.header.offset.0 as usize;
        let entries_split = entries_bytes.split(|byte| *byte == 0);
        entries_split.for_each(|segment| {
            count = count + 1;
            let offset = dtype::Off(offset_g as u64);
            let content = core::str::from_utf8(segment).unwrap();
            offset_g = offset_g + content.len();
            unsafe { *entries.add(count) = Entry { offset, content } };
        });

        let offset = string_table_entry.header.offset;

        Self {
            counter: count,
            offset,
            entries: entries,
        }
    }
}

pub struct Tables<'t> {
    pub counter: usize,
    pub table: *mut Table<'t>,
}

impl<'t> Tables<'t> {
    pub fn read_from_filepath(filepath: &str) -> Option<Self> {
        let file_descriptor = crate::open_filepath(filepath);
        Self::read_from_file_descriptor(file_descriptor)
    }

    pub fn read_from_file_descriptor(file_descriptor: isize) -> Option<Self> {
        let elf_header = crate::ELFHeader::read_from_file_descriptor(file_descriptor);
        Self::read_from_elf_header(file_descriptor, &elf_header)
    }

    pub fn read_from_elf_header(
        file_descriptor: isize,
        elf_header: &crate::ELFHeader,
    ) -> Option<Self> {
        // let mut offset = elf_header.get_shoff();
        let endianess = elf_header.get_identifier().get_endianness();
        // let number_of_entries = elf_header.get_shnum();

        let section_table = crate::SectionTable::read_from_file_descriptor(&filemap);

        Self::read_from_section_table(file_descriptor, &endianess, section_table)
    }

    pub fn read_from_section_table(
        file_descriptor: isize,
        endianess: &dtype::Endianness,
        section_table: crate::SectionTable,
    ) -> Option<Self> {
        let mut strtab_counter = 0;

        // #TODO

        let string_tables = crate::alloc::<crate::SectionEntry>(10);

        // = section_table
        // .entries
        // .into_iter()
        // .filter(|e| e.header.get_stype() == crate::section::header::Type::StrTab)
        // .collect();

        // string_tables
        //     .into_iter()
        //     .map(|entry| Self::read_from_section_header(filemap, &entry, &endianess))
        //     .collect()
        None
    }
}

impl<'t> core::fmt::Debug for Table<'t> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "String Table {{")?;
        writeln!(f, "\tOffset {:?}, ", &self.offset)?;
        writeln!(f, "\tEntries: {{")?;
        for c in 0..self.counter {
            let entry = unsafe { (*self.entries.add(c)).clone() };
            writeln!(f, "\t\t{c}:{} @ {:x}", entry.content, entry.offset.0)?;
        }
        writeln!(f, "\t}}")?;
        writeln!(f, "}}")?;
        Ok(())
    }
}

impl<'t> core::fmt::Display for Table<'t> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "{:?}", self)?;
        Ok(())
    }
}
