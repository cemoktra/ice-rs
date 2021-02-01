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