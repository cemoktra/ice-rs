use std::hash::Hash;

use crate::{errors::*, protocol::{EndPointType, EndpointData}};
use pest::{Parser, iterators::Pairs};


#[derive(Parser)]
#[grammar = "proxystring.pest"]
pub struct ProxyParser;



pub struct DirectProxyData {
    pub ident: String,
    pub endpoint: EndPointType,
}

pub struct IndirectProxyData {
    pub ident: String,
    pub adapter: Option<String>,
}

pub enum ProxyStringType {
    DirectProxy(DirectProxyData),
    IndirectProxy(IndirectProxyData)
}

pub fn parse_proxy_string(proxy_string: &str) -> Result<ProxyStringType, Box<dyn std::error::Error>> {
    let result = ProxyParser::parse(Rule::proxystring, proxy_string)?.next().unwrap();
    for child in result.into_inner() {
        match child.as_rule() {
            Rule::direct_proxy => return parse_direct_proxy(child.into_inner()),
            Rule::indirect_proxy => return parse_indirect_proxy(child.into_inner()),
            _ => {}
        }
    }
    Err(Box::new(ParsingError::new("Unexpected rule while parsing proxy string.")))
}

pub fn parse_direct_proxy(rules: Pairs<Rule>) -> Result<ProxyStringType, Box<dyn std::error::Error>> {
    let mut ident = "";
    for child in rules {
        match child.as_rule() {
            Rule::ident => {
                ident = child.as_str();
            },
            Rule::endpoint => {
                return Ok(
                    ProxyStringType::DirectProxy(
                        DirectProxyData {
                            ident: String::from(ident),
                            endpoint: parse_endpoint(child.into_inner())?
                        }
                    )
                )
            }
            _ => {}
        }
    }
    Err(Box::new(ParsingError::new("Unexpected rule while parsing proxy string.")))
}

pub fn parse_indirect_proxy(rules: Pairs<Rule>) -> Result<ProxyStringType, Box<dyn std::error::Error>> {
    let mut ident = "";
    let mut adapter = None;

    for child in rules {
        match child.as_rule() {
            Rule::ident => {
                ident = child.as_str();
            },
            Rule::adapter => {
                for child in child.into_inner() {
                    match child.as_rule() {
                        Rule::keyword_at => {}
                        Rule::ident => {
                            adapter = Some(child.as_str())
                        },
                        _ => return Err(Box::new(ParsingError::new("Unexpected rule while parsing proxy string.")))
                    }
                }
            },
            _ => return Err(Box::new(ParsingError::new("Unexpected rule while parsing proxy string.")))
        }
    }

    Ok(
        ProxyStringType::IndirectProxy(IndirectProxyData {
            ident: String::from(ident),
            adapter: if adapter.is_some() { Some(String::from(adapter.unwrap())) } else { None }
        })
    )

}

pub fn parse_endpoint(rules: Pairs<Rule>) -> Result<EndPointType, Box<dyn std::error::Error>> {
    let mut protocol = "";
    let mut host = "";
    let mut port = 0i32;

    for child in rules {
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
                            port = item.as_str().parse()?;
                        }
                        _ => return Err(Box::new(ParsingError::new(&format!("Unexpected proxy string rule: {:?}", item.as_rule()))))
                    };
                }
            }
            _ => return Err(Box::new(ParsingError::new("Unexpected rule while parsing proxy string.")))
        }
    }

    let endpoint_data = EndpointData {
        host: String::from(host),
        port,
        timeout: 60000,
        compress: false
    };

    match protocol {
        "tcp" | "default" => return Ok(EndPointType::TCP(endpoint_data)),
        "ssl" => return Ok(EndPointType::SSL(endpoint_data)),
        _ => return Err(Box::new(ParsingError::new("Unsupported protocol.")))
    }
}