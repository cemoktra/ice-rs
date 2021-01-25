use std::{collections::HashMap, hash::Hash};

use crate::{errors::*, properties::Properties};
use crate::transport::Transport;
use crate::tcp::TcpTransport;
use crate::ssl::SslTransport;
use crate::protocol::{MessageType, ReplyData, RequestData, Identity, Encapsulation};
use crate::encoding::FromBytes;
use pest::Parser;
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
    pub fn new(proxy_string: &str, properties: &Properties) -> Result<Proxy, Box<dyn std::error::Error>> { 
        let mut ident = "";
        let mut protocol = "";
        let mut host = "";
        let mut port = "";

        let result = ProxyParser::parse(Rule::proxystring, proxy_string)?.next().unwrap();
        for pair in result.into_inner() {
            match pair.as_rule() {
                Rule::ident => {
                    ident = pair.as_str();
                }
                Rule::endpoint => {
                    for child in pair.into_inner() {                        
                        match child.as_rule() {
                            Rule::endpoint_protocol => {
                                protocol = child.as_str();
                            }
                            Rule::endpoint_host | Rule::endpoint_port => {
                                for item in child.into_inner() {
                                    match item.as_rule() {
                                        Rule::hostname | Rule::ip => {
                                            host = item.as_str();
                                        }
                                        Rule::port => {
                                            port = item.as_str();
                                        }
                                        _ => return Err(Box::new(ParsingError::new(&format!("Unexpected proxy string rule: {:?}", item.as_rule()))))
                                    };
                                }
                            }
                            _ => return Err(Box::new(ParsingError::new(&format!("Unexpected proxy string rule: {:?}", child.as_rule()))))
                        };
                    }
                }
                Rule::EOI => {}
                _ => return Err(Box::new(ParsingError::new("Unexpected rule while parsing proxy string.")))
            };
        }

        let address = &(host.to_owned() + ":" + port);
        let transport: Rc<RefCell<dyn Transport>> = match protocol {
            "default" | "tcp" => Rc::new(RefCell::new(TcpTransport::new(address)?)),
            "ssl" => Rc::new(RefCell::new(SslTransport::new(address, properties)?)),
            _ => return Err(Box::new(ProtocolError::new(&format!("Unsupported protocol: {}", protocol))))
        };
        Ok(Proxy {
            transport: transport,
            request_id: 0,
            ident: String::from(ident),
            host: String::from(host),
            port: port.parse()?,
            context: None
        })
    }

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