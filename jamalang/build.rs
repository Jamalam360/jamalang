use std::{fs, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=../stdlib/src/lib.rs");

    let command = Command::new("cargo")
        .args(&["rustc", "--release", "--", "--emit=llvm-bc"])
        .current_dir("../stdlib")
        .spawn();

    command.expect("rustc should be invokeable");

    let mut files = fs::read_dir("../stdlib/target/release/deps")
        .expect("stdlib/target/release/deps should be readable")
        .into_iter();
    let path = loop {
        if let Some(Ok(file)) = files.next() {
            let file_name = file
                .file_name()
                .into_string()
                .expect("file name should be parsable");

            if file_name.starts_with("stdlib") && file_name.ends_with("bc") {
                break file.path();
            }
        }
    };

    fs::copy(path, "./src/stdlib.bc")
        .expect("stdlib.bc should be writable");
}
