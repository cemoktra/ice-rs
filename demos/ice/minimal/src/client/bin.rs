use ice_rs::communicator::Communicator;

mod gen;
use crate::gen::demo::{Hello,HelloPrx};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let comm = Communicator::new();        
    let proxy = comm.string_to_proxy("hello:default -h localhost -p 10000")?;
    let mut hello_prx = HelloPrx::checked_cast(proxy)?;

    hello_prx.say_hello(None)
}