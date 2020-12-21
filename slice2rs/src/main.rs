use ice_rs::{errors::ParsingError, slice::{
    parser
}};
use clap::Clap;
use std::path::Path;


#[derive(Clap)]
#[clap(version = "1.0")]
struct Opts {
    #[clap(short)]
    include_dir: Option<String>,
    slice_file: String,
    out_dir: String
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();
    let slice_path = Path::new(&opts.slice_file);
    let include_dir = match opts.include_dir.as_ref() {
        Some(dir) => Path::new(dir),
        None => {
            // TODO: extract fome file(s)
            Path::new(".")
        }
    };
    let root = parser::parse_ice_file(&slice_path, include_dir)?;
    root.generate(
        Path::new(&opts.out_dir),
        &slice_path
            .file_stem()
            .ok_or(Box::new(ParsingError {}))?
            .to_str()
            .ok_or(Box::new(ParsingError {}))?
            .to_lowercase()
    )
}