
//! ## Quick Start ##
//! This quick start guide will cover a client for the [ZeroC Ice Minimal Sample](https://github.com/zeroc-ice/ice-demos/tree/3.7/python/Ice/minimal). Create a binary application with `cargo new minimal-client` and add `ice-rs` to your `[build-dependencies]`and `[dependencies]`. Now add a `build.rs` file with the following content:
//!
//! ```Rust
//! use ice_rs::slice::parser;
//! use std::path::Path;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!   println!("cargo:rerun-if-changed=build.rs");
//!   let ice_files = vec![
//!       String::from("<path/to/Hello.ice>")
//!   ];
//!   let root_module = parser::parse_ice_files(&input, ".")?;
//!   root_module.generate(Path::new("./src/gen"))
//! }
//! ```
//!
//! Now add the following to you `main.rs`:
//! ```Rust
//! use ice_rs::communicator::Communicator;
//! 
//! mod gen;
//! use crate::gen::demo::{Hello,HelloPrx};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//!     let mut comm = Communicator::new().await?;
//!     let proxy = comm.string_to_proxy("hello:default -h localhost -p 10000").await?;
//!     let mut hello_prx = HelloPrx::checked_cast(proxy).await?;
//! 
//!     hello_prx.say_hello(None).await
//! }
//! ```

#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate ice_derive;

pub mod errors;
pub mod protocol;
pub mod encoding;
pub mod tcp;
pub mod ssl;
pub mod ssltools;
pub mod transport;
pub mod proxy;
pub mod proxy_parser;
pub mod proxy_factory;
pub mod communicator;
pub mod iceobject;
pub mod slice;
pub mod initdata;
pub mod properties;
pub mod locator;
pub mod adapter;