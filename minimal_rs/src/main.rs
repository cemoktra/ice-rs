use ice_rs::errors::Error;
use ice_rs::communicator::Communicator;

mod manual_gen;
use crate::manual_gen::rust_demo::{Demo,DemoPrx,Rect};



fn main() -> Result<(), Error> {
    let comm = Communicator{};
    let proxy = comm.string_to_proxy("127.0.0.1:10000")?;

    let mut demo_prx = DemoPrx::checked_cast(proxy)?;
    println!("ice_ping: {:?}", demo_prx.ice_ping());
    println!("ice_id: {:?}", demo_prx.ice_id());
    println!("ice_ids: {:?}", demo_prx.ice_ids());
    println!("ice_is_a: {:?}", demo_prx.ice_is_a());
    println!("say: {:?}", demo_prx.say("Hello from Rust"));
    println!("sayHello: {:?}", demo_prx.say_hello());

    let rc = Rect {
        left: 0,
        right: 100,
        top: 0,
        bottom: 50
    };
    println!("calcRect({:?}): {:?}", rc, demo_prx.calc_rect(rc));

    Ok(())
}
