use ice_rs::errors::*;
use ice_rs::slice::parser;
use std::path::Path;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Ice")).expect("Could not open Cargo.toml");

    let input = settings.get::<String>("slice.input").expect("Could not read slice input");
    let outdir = settings.get::<String>("slice.outdir").expect("Could not read slice output dir");

    println!("cargo:rerun-if-changed=build.rs");

    let slice_path = Path::new(&input);
    let root = parser::parse_ice_file(Path::new(&slice_path))?;
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