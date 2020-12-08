use ice_rs::errors::Error;
use ice_rs::slice::{
    parser
};
use clap::Clap;
use std::path::Path;


#[derive(Clap)]
#[clap(version = "1.0")]
struct Opts {
    slice_file: String,
    out_dir: String
}

fn main() -> Result<(), Error> {
    let opts: Opts = Opts::parse();
    let slice_path = Path::new(&opts.slice_file);
    let root = parser::parse_ice_file(Path::new(&slice_path))?;
    root.write(Path::new(&opts.out_dir), &slice_path.file_stem().ok_or(Error::ParsingError)?.to_str().ok_or(Error::ParsingError)?.to_lowercase())
}