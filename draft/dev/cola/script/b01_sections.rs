// %% Cell 01

:dep lib = { path="../../lib"}

// %% Cell 02


// %% Cell 03
println!("Hello, world!");

let data_dir_result = std::fs::read_dir("../../data");

let data_dir = data_dir_result.map_err(|e| panic!("Out: {e}")).unwrap();


for f in data_dir.into_iter() {

    let file = f.unwrap();

    let aux = std::fs::canonicalize(file.path()).unwrap();
    let aux = aux.as_path();

    println!("{:?}", aux);
}
// %% Cell  04
let filepaths = vec!{}
    "/home/zegois/lang/rust/project/rustics/dev/elf_loader/data/phnum.m68k.so",
    "/home/zegois/lang/rust/project/rustics/dev/elf_loader/data/symver.powerpc64.so",
    "/home/zegois/lang/rust/project/rustics/dev/elf_loader/data/symver.riscv64.so",
    "/home/zegois/lang/rust/project/rustics/dev/elf_loader/data/symver.aarch64.so",
    "/home/zegois/lang/rust/project/rustics/dev/elf_loader/data/symver.armhf.so",
    "/home/zegois/lang/rust/project/rustics/dev/elf_loader/data/shnum.x86_64",
    "/home/zegois/lang/rust/project/rustics/dev/elf_loader/data/symver.x86_64.so",
    "/home/zegois/lang/rust/project/rustics/dev/elf_loader/data/symver.powerpc64le.so",
    "/home/zegois/lang/rust/project/rustics/dev/elf_loader/data/symver.m68k.so",
    "/home/zegois/lang/rust/project/rustics/dev/elf_loader/data/basic.x86_64",
    "/home/zegois/lang/rust/project/rustics/dev/elf_loader/data/stripped.x86_64.so"
}
