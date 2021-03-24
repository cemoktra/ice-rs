use ice_rs::slice::parser;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Hello.ice");

    let root_module = parser::parse_ice_files(&vec![String::from("./Hello.ice")], ".")?;
    root_module.generate(Path::new("./src/gen"), "")
}