use ice_rs::errors::Error;
use ice_rs::communicator::Communicator;

mod manual_gen;
use crate::manual_gen::demo::hello::HelloPrx;
use crate::manual_gen::demo::traits::{Hello,Rect};



fn main() -> Result<(), Error> {
    let comm = Communicator{};
    let proxy = comm.string_to_proxy("127.0.0.1:10000")?;

    let mut hello_prx = HelloPrx::checked_cast(proxy)?;
    println!("ice_ping: {:?}", hello_prx.ice_ping());
    println!("ice_id: {:?}", hello_prx.ice_id());
    println!("ice_ids: {:?}", hello_prx.ice_ids());
    println!("ice_is_a: {:?}", hello_prx.ice_is_a());
    println!("say: {:?}", hello_prx.say("Hello from Rust"));
    println!("sayHello: {:?}", hello_prx.say_hello());

    let rc = Rect {
        left: 0,
        right: 100,
        top: 0,
        bottom: 50
    };
    println!("calcRect({:?}): {:?}", rc, hello_prx.calc_rect(rc));

    Ok(())
}
