# ice-rs #

The goal of this project is to support Rust in [ZeroC Ice](https://github.com/zeroc-ice/ice). Currently just client features and a small subset of ZeroC Ice features are implemented. 

## Quick Start ##
This quick start guide will cover a client for the [ZeroC Ice Minimal Sample](https://github.com/zeroc-ice/ice-demos/tree/3.7/python/Ice/minimal). Create a binary application with `cargo new minimal-client` and add `ice-rs` to your `[build-dependencies]`and `[dependencies]`. Now add a `build.rs` file with the following content:

```Rust
use ice_rs::slice::parser;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("cargo:rerun-if-changed=build.rs");
  let ice_files = vec![
      String::from("<path/to/Hello.ice>")
  ];
  let root_module = parser::parse_ice_files(&input, ".")?;
  root_module.generate(Path::new("./src/gen"))
}
```

Now add the following to you `main.rs`:
```Rust
use ice_rs::communicator::Communicator;
use ice_rs::iceobject::IceObject;

mod gen;
use crate::gen::demo::{Hello, HelloPrx};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let comm = Communicator{};
    let proxy = comm.string_to_proxy("hello:default -h localhost -p 10000")?;
    let mut hello_prx = HelloPrx::checked_cast(proxy)?;

    hello_prx.say_hello()
}
```