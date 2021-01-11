use ice_rs::slice::parser;
use clap::Clap;
use std::path::{Path, PathBuf};


#[derive(Clap)]
#[clap(version = "1.0")]
struct Opts {
    #[clap(short)]
    include_dir: Option<String>, 
    out_dir: String,
    slice_files: Vec<String>
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();
    let include_dir = match opts.include_dir {
        Some(dir) => dir,
        None => {
            let mut inc_dir = PathBuf::from(opts.slice_files.first().unwrap());
            inc_dir.pop();
            String::from(inc_dir.to_str().unwrap())
        }
    };
    let root = parser::parse_ice_files(&opts.slice_files, &include_dir)?;
    root.generate(Path::new(&opts.out_dir), "")
}