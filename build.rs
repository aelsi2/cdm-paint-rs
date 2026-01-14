use std::fs;

fn main() {
    let files = vec!["./src/ivt.asm", "./src/cdm.asm", "./src/io/io.asm"];
    for file in files {
        if let Ok(full_name) = fs::canonicalize(file) {
            println!("cargo::rustc-link-arg={}", full_name.display());
        }
    }
}
