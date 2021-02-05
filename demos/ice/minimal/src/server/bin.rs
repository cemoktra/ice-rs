use ice_rs::communicator::Communicator;
use std::collections::HashMap;
use ice_rs::protocol::*;
use ice_rs::encoding::*;
use ice_rs::iceobject::*;
use async_trait::async_trait;
mod gen;
use crate::gen::demo::{Hello};

struct HelloI {}

// TODO: move this to iceobject and add id() function that is implemented in Hello trait
#[async_trait]
impl IceObject for HelloI {
    const TYPE_ID: &'static str = "::Demo::Hello";
    async fn dispatch<
        T: 'static + std::fmt::Debug + std::fmt::Display + FromBytes + Send + Sync,
    >(
        &mut self,
        op: &str,
        mode: u8,
        params: &Encapsulation,
        context: Option<HashMap<String, String>>,
    ) -> Result<ReplyData, Box<dyn std::error::Error + Send + Sync>> {
        todo!("NOT REQUIRED BY SERVER")
    }
}

#[async_trait]
impl Hello for HelloI {
    async fn say_hello(
        &mut self,
        context: Option<HashMap<String, String>>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        println!("Hello World!");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut comm = Communicator::new().await?;

    // let mut adapter = comm.create_object_adapter().await?;
    // let mut adapter = comm.create_object_adapter_with_endpoints("Hello", "default -h localhost -p 10000").await?;
    // adapter.add(Box::new(HelloI{}), "hello").await?;
    // adapter.activate().await?;

    // comm.wait_for_shutdown().await?;

    Ok(())
}