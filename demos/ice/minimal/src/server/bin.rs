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

    // comm.wait_for_shutdown().await?;

    Ok(())
}