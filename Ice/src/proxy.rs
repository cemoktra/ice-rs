use crate::errors::*;
use crate::transport::Transport;
use crate::tcp::TcpTransport;
use crate::protocol::{MessageType, ReplyData, RequestData, Identity, Encapsulation};
use crate::encoding::FromBytes;

pub struct Proxy {
    pub transport: Box<dyn Transport + 'static>,
    pub request_id: i32
}

impl Proxy {
    pub fn new(proxy_string: &str) -> Result<Proxy, Box<dyn std::error::Error>> {
        // TODO: parse real proxy string
        Ok(Proxy {
            transport: Box::new(TcpTransport::new(proxy_string)?),
            request_id: 0
        })
    }

    pub fn create_request(&mut self, identity_name: &str, operation: &str, mode: u8, params: &Encapsulation) -> RequestData {
        self.request_id = self.request_id + 1;
        RequestData {
            request_id: self.request_id,
            id: Identity {
                name: String::from(identity_name),
                category: String::from("")
            },
            facet: Vec::new(),
            operation: String::from(operation),
            mode: mode,
            context: std::collections::HashMap::new(),
            params: params.clone()
        }
    }

    pub fn make_request<T: 'static + std::fmt::Debug + std::fmt::Display + FromBytes>(&mut self, request: &RequestData) -> Result<ReplyData, Box<dyn std::error::Error>>
    {
        self.transport.make_request(request)?;
        let reply = self.transport.read_message()?;
        match reply {
            MessageType::Reply(_header, reply) => {
                match reply.status {
                    1 => {
                        let mut read = 0;
                        Err(Box::new(UserError {
                            exception: T::from_bytes(&reply.body.data, &mut read)?
                        }))
                    }
                    _ => Ok(reply)
                }
            },
            _ => Err(Box::new(ProtocolError {}))
        }
    }
}