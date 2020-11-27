use ice_rs::protocol::{Encapsulation, Identity, RequestData};
use ice_rs::errors::Error;
use ice_rs::proxy::Proxy;

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
    proxy: Proxy,
    name: String,
    type_id: String
}

impl Hello for HelloPrx {
    fn ice_is_a(&mut self) -> Result<bool, Error> {
        let req = RequestData {
            request_id: 1,
            id: Identity {
                name: self.name.clone(),
                category: String::from("")
            },
            facet: Vec::new(),
            operation: String::from("ice_isA"),
            mode: 1,
            context: std::collections::HashMap::new(),
            params: Encapsulation {
                size: 20,
                major: 1,
                minor: 1,
                data: self.type_id.as_bytes().to_vec()
            }
        };
    
        let reply = self.proxy.make_request(&req)?;
        if reply.body.data.len() == 1 {
            Ok(reply.body.data[0] != 0)
        } else {
            Ok(false)
        }
    }

    fn say_hello(&mut self) -> Result<(), Error> {
        let req = RequestData {
            request_id: 1,
            id: Identity {
                name: self.name.clone(),
                category: String::from("")
            },
            facet: Vec::new(),
            operation: String::from("sayHello"),
            mode: 0,
            context: std::collections::HashMap::new(),
            params: Encapsulation {
                size: 6,
                major: 1,
                minor: 1,
                data: vec![]
            }
        };
    
        self.proxy.make_request(&req)?;
        Ok(())
    }
}

impl HelloPrx {
    fn new(proxy_string: &str) -> Result<HelloPrx, Error> {
        let mut hello_proxy = HelloPrx {
            proxy: Proxy::new(proxy_string)?,
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
    let mut hello = HelloPrx::new("127.0.0.1:10000")?;
    hello.say_hello()
}
