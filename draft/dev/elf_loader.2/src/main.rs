use lib;

fn main() {
    println!("Hello, world!");

    let data_dir_result = std::fs::read_dir("./data");

    let data_dir = data_dir_result.map_err(|e| panic!("Out: {e}")).unwrap();

    for f in data_dir.into_iter() {
        let file = f.unwrap();

        let file_output = std::process::Command::new("sh")
            .arg("-c")
            .arg(format!("file {:?}", file.path()))
            .output()
            .expect("Cannot file the file");

        let file_output = String::from_utf8_lossy(&file_output.stdout);

        let aux = std::fs::canonicalize(file.path()).unwrap();
        let aux = aux.as_path();

        println!("{:?}", file_output);

        /*
        if file_output.contains("32-bit") {
            let elf = lib::ELF32::load_from_filepath(aux);

            let file_output = std::process::Command::new("sh")
                .arg("-c")
                .arg(format!("readelf -h {:?}", file.path()))
                .output()
                .expect("Cannot file the file");

            let file_output = String::from_utf8_lossy(&file_output.stdout);

            println!("{}", file_output);
        }
        */

        if file_output.contains("64-bit") {
            let elf = lib::ELF64::load_from_filepath(aux);
        }
    }
}
