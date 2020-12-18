use ice_rs::communicator::Communicator;
use ice_rs::iceobject::IceObject;

mod gen;
use crate::gen::rust_demo::{Demo,DemoPrx,Rect};
use std::collections::HashMap;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let comm = Communicator{};
    let proxy = comm.string_to_proxy("127.0.0.1:10000")?;

    let mut demo_prx = DemoPrx::checked_cast(proxy)?;
    println!("ice_ping: {:?}", IceObject::ice_ping(&mut demo_prx));
    println!("ice_id: {:?}", IceObject::ice_id(&mut demo_prx));
    println!("ice_ids: {:?}", IceObject::ice_ids(&mut demo_prx));
    println!("ice_is_a: {:?}", IceObject::ice_is_a(&mut demo_prx));
    println!("say: {:?}", demo_prx.say(&String::from("Hello from Rust")));
    println!("sayHello: {:?}", demo_prx.say_hello());

    let rc = Rect {
        left: 0,
        right: 100,
        top: 0,
        bottom: 50
    };
    println!("calcRect({:?}): {:?}", rc, demo_prx.calc_rect(&rc));

    println!("add: {:?}", demo_prx.add(3.0, 4.5));

    let mut x = 4.0; 
    println!("square: {:?}", demo_prx.square(x, &mut x));
    println!("  x = {}", x);
    println!("squareRoot: {:?}", demo_prx.square_root(x, &mut x));
    println!("  x = {}", x);

    let y: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
    println!("sum: {:?}", demo_prx.sum(&y));

    let mut z: HashMap<String, f64> = HashMap::new();
    z.insert(String::from("hello"), 1.0);
    z.insert(String::from("world"), 2.0);
    println!("getHello: {:?}", demo_prx.get_hello(&z));

    println!("nativeException: {:?}", demo_prx.native_exception());
    println!("baseException: {:?}", demo_prx.base_exception());
    println!("derivedException: {:?}", demo_prx.derived_exception());

    Ok(())
}
