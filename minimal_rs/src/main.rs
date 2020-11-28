use ice_rs::protocol::{Encapsulation};
use ice_rs::errors::Error;
use ice_rs::proxy::Proxy;
use std::rc::Rc;
use std::cell::RefCell;

// The trait and implementaion is done manually
// but demonstrates calling sayHello on the minimal ice demo
trait Hello {
    // base ice
    fn ice_is_a(&mut self) -> Result<bool, Error>;
    // hello interface
    fn say_hello(&mut self) -> Result<(), Error>;
}

struct HelloPrx
{
    proxy: Rc<RefCell<Proxy>>,
    name: String,
    type_id: String
}

impl Hello for HelloPrx {
    fn ice_is_a(&mut self) -> Result<bool, Error> {
        let req = self.proxy.borrow_mut().create_request(
            &self.name, 
            &String::from("ice_isA"),
            1, 
            Encapsulation::new(&self.type_id.as_bytes().to_vec())
        );
        let reply = self.proxy.borrow_mut().make_request(&req)?;
        if reply.body.data.len() == 1 {
            Ok(reply.body.data[0] != 0)
        } else {
            Ok(false)
        }
    }

    fn say_hello(&mut self) -> Result<(), Error> {
        let req = self.proxy.borrow_mut().create_request(
            &self.name, 
            &String::from("sayHello"),
            0, 
            Encapsulation::new(&vec![])
        );    
        self.proxy.borrow_mut().make_request(&req)?;
        Ok(())
    }
}

impl HelloPrx {
    fn checked_cast(proxy: Rc<RefCell<Proxy>>) -> Result<HelloPrx, Error> {
        let mut hello_proxy = HelloPrx {
            proxy: proxy,
            name: String::from("hello"),
            type_id: String::from("\r::Demo::Hello")
        };

        if !hello_proxy.ice_is_a()? {
            return Err(Error::TcpCannotConnect);
        }

        Ok(hello_proxy)
    }
}

fn main() -> Result<(), Error> {
    // TODO: add Communicator with stringToProxy
    let proxy = Proxy::new("127.0.0.1:10000")?;

    let mut hello_prx = HelloPrx::checked_cast(proxy)?;
    hello_prx.say_hello()
}
