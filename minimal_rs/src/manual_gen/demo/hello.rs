use super::traits::Hello;
use ice_rs::{errors::Error, protocol::deserialize_string_seq};
use ice_rs::proxy::Proxy;
use ice_rs::protocol::{Encapsulation};

pub struct HelloPrx
{
    proxy: Proxy
}

impl Hello for HelloPrx {
    fn ice_ping(&mut self) -> Result<(), Error>
    {
        let req = self.proxy.create_request(
            &HelloPrx::NAME, 
            &String::from("ice_ping"),
            0, 
            Encapsulation::empty()
        );    
        self.proxy.make_request(&req)?;
        Ok(())
    }

    fn ice_is_a(&mut self) -> Result<bool, Error> {
        let req = self.proxy.create_request(
            &HelloPrx::NAME, 
            &String::from("ice_isA"),
            1, 
            Encapsulation::new(&HelloPrx::TYPE_ID.as_bytes().to_vec())
        );
        let reply = self.proxy.make_request(&req)?;
        if reply.body.data.len() == 1 {
            Ok(reply.body.data[0] != 0)
        } else {
            Ok(false)
        }
    }

    fn ice_id(&mut self) -> Result<String, Error>
    {
        let req = self.proxy.create_request(
            &HelloPrx::NAME, 
            &String::from("ice_id"),
            1, 
            Encapsulation::empty()
        );
        let reply = self.proxy.make_request(&req)?;
        Ok(String::from_utf8(reply.body.data)?)
    }

    fn ice_ids(&mut self) -> Result<Vec<String>, Error>
    {
        let req = self.proxy.create_request(
            &HelloPrx::NAME, 
            &String::from("ice_ids"),
            1, 
            Encapsulation::empty()
        );
        let reply = self.proxy.make_request(&req)?;
        deserialize_string_seq(&reply.body.data)
    }

    fn say_hello(&mut self) -> Result<(), Error> {
        let req = self.proxy.create_request(
            &HelloPrx::NAME, 
            &String::from("sayHello"),
            0, 
            Encapsulation::empty()
        );    
        self.proxy.make_request(&req)?;
        Ok(())
    }
}

impl HelloPrx {
    const TYPE_ID: &'static str = "\r::Demo::Hello";
    const NAME: &'static str = "hello";

    pub fn checked_cast(proxy: Proxy) -> Result<HelloPrx, Error> {
        let mut hello_proxy = HelloPrx {
            proxy: proxy
        };

        if !hello_proxy.ice_is_a()? {
            return Err(Error::TcpCannotConnect);
        }

        Ok(hello_proxy)
    }
}