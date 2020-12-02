use ice_rs::errors::Error;
use ice_rs::communicator::Communicator;
use ice_rs::iceobject::IceObject;

mod manual_gen;
use crate::manual_gen::rust_demo::{Demo,DemoPrx,Rect};



fn main() -> Result<(), Error> {
    let comm = Communicator{};
    let proxy = comm.string_to_proxy("127.0.0.1:10000")?;

    let mut demo_prx = DemoPrx::checked_cast(proxy)?;
    println!("ice_ping: {:?}", IceObject::ice_ping(&mut demo_prx));
    println!("ice_id: {:?}", IceObject::ice_id(&mut demo_prx));
    println!("ice_ids: {:?}", IceObject::ice_ids(&mut demo_prx));
    println!("ice_is_a: {:?}", IceObject::ice_is_a(&mut demo_prx));
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
