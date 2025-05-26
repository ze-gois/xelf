pub mod data;
pub mod header;
pub mod table;

pub struct ELF {
    pub filemap: memmap2::Mmap,
    pub header: header::ELF,
    pub program_header_table: Option<table::Program>,
    pub section_header_table: Option<table::Section>,
}

impl ELF {
    pub fn load_from_filepath<P: AsRef<std::path::Path>>(filepath: P) -> Self {
        let file = std::fs::File::open(filepath).unwrap();
        let filemap = unsafe { memmap2::Mmap::map(&file).unwrap() };

        let elf_header = header::ELF::load_from_memmap(&filemap);

        let program_header_table = match elf_header.get_phoff() {
            0 => None,
            _ => Some(table::Program::load_from_elf_header(&filemap, &elf_header)),
        };

        let section_header_table = match elf_header.get_shoff() {
            0 => None,
            _ => Some(table::Section::load_from_elf_header(&filemap, &elf_header)),
        };

        Self {
            filemap,
            header: elf_header,
            program_header_table,
            section_header_table,
        }
    }
}
