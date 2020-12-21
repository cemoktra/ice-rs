use ice_rs::errors::*;
use ice_rs::slice::parser;
use std::path::Path;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Ice")).expect("Could not open Cargo.toml");

    let input = settings.get::<Vec<String>>("slice.input").expect("Could not read slice input");
    let outdir = settings.get::<String>("slice.outdir").expect("Could not read slice output dir");
    let include_dir = match settings.get::<String>("slice.include_dir") {
        Ok(dir) => dir,
        _ => {
            // TODO: extract fome file(s)
            String::from(".")
        }
    };

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Ice.toml");

    let root = parser::parse_ice_files(&input, &include_dir)?;
    root.generate(Path::new(&outdir))
}