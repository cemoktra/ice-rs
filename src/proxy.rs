use std::{collections::HashMap, hash::Hash};

use crate::errors::*;
use crate::transport::Transport;
use crate::protocol::{MessageType, ReplyData, RequestData, Identity, Encapsulation};
use crate::encoding::FromBytes;
use std::rc::Rc;
use std::cell::RefCell;


#[derive(Parser)]
#[grammar = "proxystring.pest"]
pub struct ProxyParser;


pub struct Proxy {
    pub transport: Rc<RefCell<dyn Transport + 'static>>,
    pub request_id: i32,
    pub ident: String,
    pub host: String,
    pub port: i32,
    pub context: Option<HashMap<String, String>>
}

impl Proxy {
    pub fn ice_context(&mut self, context: HashMap<String, String>) -> Proxy {
        Proxy {
            transport: self.transport.clone(),
            request_id: self.request_id,
            ident: self.ident.clone(),
            host: self.host.clone(),
            port: self.port,
            context: Some(context)
        }
    }

    pub fn create_request(&mut self, identity_name: &str, operation: &str, mode: u8, params: &Encapsulation, context: Option<HashMap<String, String>>) -> RequestData {
        let context = match context {
            Some(context) => context,
            None => {
                match self.context.as_ref() {
                    Some(context) => context.clone(),
                    None => HashMap::new()
                }
            }
        };
        self.request_id = self.request_id + 1;
        RequestData {
            request_id: self.request_id,
            id: Identity::new(identity_name),
            facet: Vec::new(),
            operation: String::from(operation),
            mode: mode,
            context: context,
            params: params.clone()
        }
    }

    pub fn make_request<T: 'static + std::fmt::Debug + std::fmt::Display + FromBytes>(&mut self, request: &RequestData) -> Result<ReplyData, Box<dyn std::error::Error>>
    {
        let mut tx = self.transport.borrow_mut();
        tx.make_request(request)?;
        let reply = tx.read_message()?;
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
            _ => Err(Box::new(ProtocolError::new(&format!("Unsupported message type: {:?}", reply))))
        }
    }
}
// Ice::ObjectNotFoundException