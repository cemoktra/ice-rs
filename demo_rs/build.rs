use ice_rs::errors::Error;
use ice_rs::slice::parser;
use std::path::Path;


fn main() -> Result<(), Error> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Cargo")).expect("Could not open Cargo.toml");

    let input = settings.get::<String>("slice.input").expect("Could not read slice input");
    let outdir = settings.get::<String>("slice.outdir").expect("Could not read slice output dir");

    println!("cargo:rerun-if-changed=build.rs");

    let slice_path = Path::new(&input);
    let root = parser::parse_ice_file(Path::new(&slice_path))?;
    root.write(
        Path::new(&outdir),
        &slice_path
            .file_stem()
            .ok_or(Error::ParsingError)?
            .to_str()
            .ok_or(Error::ParsingError)?
            .to_lowercase()
    )
}