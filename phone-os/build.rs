use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    
    // Tell cargo to recompile if assembly changes
    println!("cargo:rerun-if-changed=src/arch/aarch64/start.S");
    
    // Assemble the file using aarch64-linux-gnu-as
    let asm_file = "src/arch/aarch64/start.S";
    let obj_file = out_dir.join("start.o");
    
    Command::new("aarch64-linux-gnu-as")
        .arg("-o")
        .arg(&obj_file)
        .arg(asm_file)
        .status()
        .expect("Failed to run assembler");
    
    // Link the object file directly
    println!("cargo:rustc-link-arg={}", obj_file.display());
}
