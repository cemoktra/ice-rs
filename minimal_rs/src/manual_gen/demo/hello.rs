use super::traits::Hello;
use ice_rs::{errors::Error, protocol::ReplyData, protocol::Encapsulation};
use ice_rs::proxy::Proxy;
use ice_rs::encoding::{FromEncapsulation, AsEncapsulation};

pub struct HelloPrx
{
    proxy: Proxy
}

impl Hello for HelloPrx {
    fn ice_ping(&mut self) -> Result<(), Error>
    {
        self.dispatch(&String::from("ice_ping"), 1, Encapsulation::empty())?;
        Ok(())
    }

    fn ice_is_a(&mut self) -> Result<bool, Error> {
        let reply = self.dispatch(&String::from("ice_isA"), 1, HelloPrx::TYPE_ID.as_encapsulation()?)?;
        bool::from_encapsulation(reply.body)
    }

    fn ice_id(&mut self) -> Result<String, Error>
    {
        let reply = self.dispatch(&String::from("ice_id"), 1, Encapsulation::empty())?;
        String::from_encapsulation(reply.body)
    }

    fn ice_ids(&mut self) -> Result<Vec<String>, Error>
    {
        let reply = self.dispatch(&String::from("ice_ids"), 1, Encapsulation::empty())?;
        Vec::from_encapsulation(reply.body)
    }

    fn say_hello(&mut self) -> Result<(), Error> {
        self.dispatch(&String::from("sayHello"), 0, Encapsulation::empty())?;
        Ok(())
    }

    fn say(&mut self, text: &str) -> Result<(), Error> {
        self.dispatch(&String::from("say"), 0, text.as_encapsulation()?)?;
        Ok(())
    }
}

impl HelloPrx {
    const TYPE_ID: &'static str = "::Demo::Hello";
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

    fn dispatch(&mut self, op: &str, mode: u8, params: Encapsulation) -> Result<ReplyData, Error> {
        let req = self.proxy.create_request(
            &HelloPrx::NAME, 
            op,
            mode, 
            params
        );
        self.proxy.make_request(&req)
    }
}