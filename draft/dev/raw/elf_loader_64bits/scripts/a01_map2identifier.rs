// %% Celula 1
:dep memmap2
:dep ellib = { path = "../lib" }

// %% Celula 2
let filepath: &str = "/home/zegois/lang/rust/project/rustics/exp/hello_32_64/target/x86_64-unknown-linux-gnu/debug/hello_32_64";
let elf_file = std::fs::File::open(filepath).unwrap();
let elf_file_map = unsafe { memmap2::Mmap::map(&elf_file).unwrap() };

// %% Celula 4
use ellib::types::header::elf::Identifier;

// %% Celula 5
let x = unsafe { core::slice::from_raw_parts(elf_file_map.as_ptr() as *const Identifier,1) };

println!("{}",x[0]);
