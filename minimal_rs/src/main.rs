use ice_rs::errors::Error;
use ice_rs::communicator::Communicator;

mod manual_gen;
use crate::manual_gen::demo::hello::HelloPrx;
use crate::manual_gen::demo::traits::Hello;



fn main() -> Result<(), Error> {
    let comm = Communicator{};
    let proxy = comm.string_to_proxy("127.0.0.1:10000")?;

    let mut hello_prx = HelloPrx::checked_cast(proxy)?;
    hello_prx.say_hello()
}
