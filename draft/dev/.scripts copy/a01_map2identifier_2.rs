// %% Celula 1
:dep memmap2
:dep ellib = { path = "../lib" }

// %% Celula 2
let filepath: &str = "/home/zegois/lang/rust/project/rustics/exp/hello_32_64/target/x86_64-unknown-linux-gnu/debug/hello_32_64";
let elf_file = std::fs::File::open(filepath).unwrap();
let elf_file_map = unsafe { memmap2::Mmap::map(&elf_file).unwrap() };

// %% Celula 4
use ellib::types::header::elf::Identifier;
use ellib::types::header::elf::Header64;

// %% Cell

core::mem::size_of::<Identifier>()

// %% Cell

core::mem::size_of::<Header64>()

// %% Celula 5

elf_file_map.as_ptr() as *const Header64
