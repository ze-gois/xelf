mod types;

use memmap2;

fn main() -> std::io::Result<()> {
    let file_path_str: &str = "/home/zegois/lang/rust/project/rustics/exp/hello_32_64/target/x86_64-unknown-linux-gnu/debug/hello_32_64";
    let file_path = std::path::Path::new(file_path_str);
    let file = std::fs::File::open(file_path).unwrap();

    let file_map = unsafe { memmap2::Mmap::map(&file).unwrap() };

    let e_ident: Vec<&u8> = file_map.iter().take(16).collect();

    for (e, ei) in e_ident.into_iter().enumerate() {
        println!("{e}:\t{:?}", *ei);
    }

    Ok(())
}
