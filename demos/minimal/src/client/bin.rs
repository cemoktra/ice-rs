use ice_rs::communicator::Communicator;

mod gen;
use crate::gen::demo::{Hello,HelloPrx};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let comm = Communicator{};
    let proxy = comm.string_to_proxy("127.0.0.1:10000")?;
    let mut hello_prx = HelloPrx::checked_cast("hello", proxy)?;

    hello_prx.say_hello()
}