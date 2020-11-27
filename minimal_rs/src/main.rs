use ice_rs::protocol::{Encapsulation, Identity, RequestData};
use ice_rs::transport::Transport;
use ice_rs::tcp::TcpTransport;
use ice_rs::errors::Error;

// The trait and implementaion is done manually
// but demonstrates calling sayHello on the minimal ice demo
trait Hello {
    fn say_hello(&self) -> Result<(), Error>;
}

struct HelloI;

impl Hello for HelloI {
    fn say_hello(&self) -> Result<(), Error> {
        let mut transport = TcpTransport::new("127.0.0.1:10000")?;
        transport.validate_connection()?;
    
        let req = RequestData {
            request_id: 1,
            id: Identity {
                name: String::from("hello"),
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
                data: String::from("\r::Demo::Hello").as_bytes().to_vec()
            }
        };
    
        transport.make_request(&req)?;
        let response = transport.read_message()?;
        println!("{:?}", response);
    
    
    
        let req = RequestData {
            request_id: 2,
            id: Identity {
                name: String::from("hello"),
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
    
        transport.make_request(&req)?;
        let response = transport.read_message()?;
        println!("{:?}", response);

        Ok(())
    }
}


fn main() -> Result<(), Error> {
    let hello = HelloI {};
    hello.say_hello()
}
