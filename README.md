[![Build Status](https://github.com/cemoktra/ice-rs/workflows/CI/badge.svg)](https://github.com/cemoktra/ice-rs/actions)

# ice-rs #

The goal of this project is to support Rust in [ZeroC Ice](https://github.com/zeroc-ice/ice). 

## Quick Start ##
This quick start guide will cover a client for the [ZeroC Ice Minimal Sample](https://github.com/zeroc-ice/ice-demos/tree/3.7/python/Ice/minimal). Create a binary application with `cargo new minimal-client` and add `ice-rs` to your `[build-dependencies]`and `[dependencies]`. Now add a `build.rs` file with the following content:


### Minimal client ###
```Rust
use ice_rs::slice::parser;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("cargo:rerun-if-changed=build.rs");
  let ice_files = vec![
      String::from("<path/to/Hello.ice>")
  ];
  let root_module = parser::parse_ice_files(&ice_files, "<path/to/ice/include/dir>")?;
  root_module.generate(Path::new("./src/gen"), "")
}
```

Now add the following to you `main.rs`:
```Rust
use ice_rs::communicator::Communicator;

mod gen;
use crate::gen::demo::{Hello,HelloPrx};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut comm = Communicator::new().await?;
    let proxy = comm.string_to_proxy("hello:default -h localhost -p 10000").await?;
    let mut hello_prx = HelloPrx::checked_cast(proxy).await?;

    hello_prx.say_hello(None).await
}
```

### Minimal server ###
Based on the same `build.rs` file you can add a server for the minimal example.

```Rust
use ice_rs::communicator::Communicator;
use std::collections::HashMap;
use async_trait::async_trait;

mod gen;
use crate::gen::demo::{HelloServer, HelloI};

struct HelloImpl {}

#[async_trait]
impl HelloI for HelloImpl {
    async fn say_hello(&mut self, _context: Option<HashMap<String, String>>) -> ()
    {
        println!("Hello World!");
        ()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let comm = Communicator::new().await?;
    let mut adapter = comm.create_object_adapter_with_endpoint("hello", "tcp -h localhost -p 10000").await?;

    let hello_server = HelloServer::new(Box::new(HelloImpl{}));

    adapter.add("hello", Box::new(hello_server));
    adapter.activate().await?;
    
    Ok(())
}
```

## Status ##
The status can be seen in the number of supported [ZeroC Ice Demos](http://github.com/zeroc-ice/ice-demos). 

- Ice/minimal
- Ice/optional
- Ice/context (implicit context missing, see [Issue](https://github.com/cemoktra/ice-rs/issues/37))
- IceGrid/simple

Supported transports:
- TCP
- SSL


## Roadmap ##
The main goal is to support all [ZeroC Ice Demos](http://github.com/zeroc-ice/ice-demos).
