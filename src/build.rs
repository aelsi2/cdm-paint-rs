use std::fs;

static LINKER_FILES: &[&str] = &[ 
    "./src/ivt.asm",
    "./src/cdm.asm",
    "./src/io.asm",
];

fn main() {
    for file in LINKER_FILES {
        println!("cargo::rustc-link-arg={}", fs::canonicalize(file).unwrap().display());
    }
}
