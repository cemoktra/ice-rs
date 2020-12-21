use ice_rs::errors::*;
use ice_rs::slice::parser;
use std::path::Path;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Ice")).expect("Could not open Cargo.toml");

    let input = settings.get::<String>("slice.input").expect("Could not read slice input");
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

    let slice_path = Path::new(&input);
    let root = parser::parse_ice_file(Path::new(&slice_path), Path::new(&include_dir))?;
    root.generate(
        Path::new(&outdir),
        &slice_path
            .file_stem()
            .ok_or(Box::new(ParsingError {}))?
            .to_str()
            .ok_or(Box::new(ParsingError {}))?
            .to_lowercase()
    )
}