use crate::errors::Error;
use crate::transport::Transport;
use crate::tcp::TcpTransport;
use crate::protocol::{MessageType, ReplyData, RequestData};

pub struct Proxy {
    pub transport: Box<dyn Transport + 'static>
}

impl Proxy {
    pub fn new(proxy_string: &str) -> Result<Proxy, Error> {
        // TODO: parse real proxy string
        Ok(Proxy {
            transport: Box::new(TcpTransport::new(proxy_string)?)
        })
    }

    pub fn make_request(&mut self, request: &RequestData) -> Result<ReplyData, Error>
    {
        self.transport.make_request(request)?;
        let reply = self.transport.read_message()?;
        match reply {
            MessageType::Reply(_header, reply) => {
                Ok(reply)
            },
            // TODO: create error
            _ => Err(Error::UnknownMessageType) 
        }
    }
}