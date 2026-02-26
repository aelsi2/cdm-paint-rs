fn main() {
    let args = vec!["-Tlink.ld", "src/ivt.asm", "src/cdm.asm", "src/io/io.asm"];
    for arg in args {
        println!("cargo::rustc-link-arg={}", arg);
    }
}
