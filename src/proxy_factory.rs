use std::cell::RefCell;
use std::rc::Rc;

use crate::{errors::ProtocolError, locator::Locator, properties::Properties, protocol::EndPointType, proxy::Proxy, proxy_parser::{DirectProxyData, ProxyStringType, parse_proxy_string}, ssl::SslTransport, tcp::TcpTransport};

pub struct ProxyFactory {
    locator: Option<Locator>
}

impl ProxyFactory {
    fn create_proxy(proxy_data: DirectProxyData, properties: &Properties) -> Result<Proxy, Box<dyn std::error::Error>> {
        match proxy_data.endpoint {
            EndPointType::TCP(endpoint) => {
                Ok(Proxy {
                    transport: Rc::new(RefCell::new(TcpTransport::new(&format!("{}:{}", endpoint.host, endpoint.port))?)),
                    request_id: 0,
                    ident: proxy_data.ident,
                    host: endpoint.host,
                    port: endpoint.port,
                    context: None
                })
            }
            EndPointType::SSL(endpoint) => {
                Ok(Proxy {
                    transport: Rc::new(RefCell::new(SslTransport::new(&format!("{}:{}", endpoint.host, endpoint.port), properties)?)),
                    request_id: 0,
                    ident: proxy_data.ident,
                    host: endpoint.host,
                    port: endpoint.port,
                    context: None
                })
            }
            _ => Err(Box::new(ProtocolError::new(&format!("Error creating proxy"))))
        }
    }

    pub fn new(properties: &Properties) -> Result<ProxyFactory, Box<dyn std::error::Error>> {
        Ok(ProxyFactory {
            locator: match properties.get("Ice.Default.Locator") {
                Some(locator_proxy) => {
                    match parse_proxy_string(locator_proxy) {
                        Ok(proxy_type) => {
                            match proxy_type {
                                ProxyStringType::DirectProxy(data) => {
                                    Some(Locator::from(ProxyFactory::create_proxy(data, properties)?))
                                }
                                _ => None
                            }
                        },
                        _ => None
                    }
                },
                _ => None
            }
        })
    }

    pub fn create(&mut self, proxy_string: &str, properties: &Properties) -> Result<Proxy, Box<dyn std::error::Error>> {
        match parse_proxy_string(proxy_string)? {
            ProxyStringType::DirectProxy(data) => {
                ProxyFactory::create_proxy(data, properties)
            }
            ProxyStringType::IndirectProxy(data) => {
                match self.locator.as_mut() {
                    Some(locator) => {
                        let data = locator.locate(data)?;
                        ProxyFactory::create_proxy(data, properties)
                    }
                    _ => Err(Box::new(ProtocolError::new(&format!("No locator set up to resolve indirect proxy"))))
                }
            }
        }
    }
}