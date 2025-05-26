use ellib::types;

fn main() -> std::io::Result<()> {
    let filepath_str: &str = "/home/zegois/lang/rust/project/rustics/exp/hello_32_64/target/x86_64-unknown-linux-gnu/debug/hello_32_64";
    let elf_file = std::fs::File::open(filepath_str).unwrap();
    let elf_filemap = unsafe { ellib::memmap2::Mmap::map(&elf_file).unwrap() };

    let h_loaded = types::header::ELFHeader64::load_from_filemap(&elf_filemap);
    let h_casted = unsafe { &*(elf_filemap.as_ptr() as *const types::ELFHeader64) };

    println!("{h_loaded}");
    println!("--------------------------");
    println!("{h_casted}");

    let output = std::process::Command::new("zsh")
        .arg("-c")
        .arg(format!("file {}", filepath_str))
        .output()
        .expect("Cannot file the file");

    println!("{:?}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}
