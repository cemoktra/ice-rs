use crate::{errors::*, properties::Properties};
use crate::transport::Transport;
use crate::tcp::TcpTransport;
use crate::ssl::SslTransport;
use crate::protocol::{MessageType, ReplyData, RequestData, Identity, Encapsulation};
use crate::encoding::FromBytes;
use pest::Parser;

#[derive(Parser)]
#[grammar = "proxystring.pest"]
pub struct ProxyParser;


pub struct Proxy {
    pub transport: Box<dyn Transport + 'static>,
    pub request_id: i32,
    pub ident: String,
    pub host: String,
    pub port: i32
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
                    // TODO: add proxy options
                    // https://doc.zeroc.com/ice/3.7/client-side-features/proxies/proxy-and-endpoint-syntax
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
                                        _ => return Err(Box::new(ProtocolError {}))
                                    };
                                }
                            }
                            _ => return Err(Box::new(ProtocolError {}))
                        };
                    }
                }
                Rule::EOI => {}
                _ => return Err(Box::new(ProtocolError {}))
            };
        }

        let address = &(host.to_owned() + ":" + port);
        match protocol {
            "default" | "tcp" => {
                Ok(Proxy {
                    transport: Box::new(TcpTransport::new(address)?),
                    request_id: 0,
                    ident: String::from(ident),
                    host: String::from(host),
                    port: port.parse()?

                })
            }
            "ssl" => {
                Ok(Proxy {
                    transport: Box::new(SslTransport::new(address, properties)?),
                    request_id: 0,
                    ident: String::from(ident),
                    host: String::from(host),
                    port: port.parse()?
                })
            }
            _ => return Err(Box::new(ProtocolError {}))
        }
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