use std::collections::HashMap;
use crate::encoding::IceSize;


#[derive(Debug)]
pub enum MessageType {
    // Request,
    // BatchRequest,
    Reply(Header, ReplyData),
    ValidateConnection(Header),
    // CloseConnection
}

#[derive(Debug)]
pub struct Header {
    pub magic: String,
    pub protocol_major: u8,
    pub protocol_minor: u8,
    pub encoding_major: u8,
    pub encoding_minor: u8,
    pub message_type: u8,
    pub compression_status: u8,
    pub message_size: i32
}

#[derive(Debug)]
pub struct Identity {
    pub name: String,
    pub category: String
}

impl Identity {
    pub fn new(ident: &str) -> Identity {
        match ident.find("/") {
            Some(_) => {
                let split = ident.split("/").collect::<Vec<&str>>();
                Identity {
                    name: String::from(split[1]),
                    category: String::from(split[0])
                }
            }
            None => Identity {
                name: String::from(ident),
                category: String::new()
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Encapsulation {
    pub size: i32,
    pub major: u8,
    pub minor: u8,
    pub data: Vec<u8>
}

#[derive(Debug)]
pub struct RequestData {
    pub request_id: i32,
    pub id: Identity,
    pub facet: Vec<String>,
    pub operation: String,
    pub mode: u8,
    pub context: HashMap<String, String>,
    pub params: Encapsulation
}

#[derive(Debug)]
pub struct ReplyData {
    pub request_id: i32,
    pub status: u8,
    pub body: Encapsulation
}

#[derive(Debug)]
pub struct Version
{
    pub major: u8,
    pub minor: u8
}

#[derive(Debug)]
pub struct ProxyData {
    pub id: String,
    pub facet: Vec<String>,
    pub mode: u8,
    pub secure: bool,
    pub protocol: Version,
    pub encoding: Version
}

#[derive(Debug)]
pub enum EndPointType {
    WellKnownObject(String),
    TCP(TCPEndpointData),
    SSL(SSLEndpointData),
}

#[derive(Debug)]
pub struct LocatorResult {
    pub proxy_data: ProxyData,
    pub size: IceSize,
    pub endpoint: EndPointType
}

#[derive(Debug)]
pub struct TCPEndpointData
{
    pub host: String,
    pub port: i32,
    pub timeout: i32,
    pub compress: bool
}

#[derive(Debug)]
pub struct SSLEndpointData
{
    pub host: String,
    pub port: i32,
    pub timeout: i32,
    pub compress: bool
}

impl Header {
    pub fn new(message_type: u8, message_size: i32) -> Header {
        Header {
            magic: String::from("IceP"),
            protocol_major: 1,
            protocol_minor: 0,
            encoding_major: 1,
            encoding_minor: 0,
            message_type: message_type,
            compression_status: 0,
            message_size: message_size
        }
    }
}

impl Encapsulation {
    pub fn empty() -> Encapsulation {
        Encapsulation {
            size: 6,
            major: 1,
            minor: 1,
            data: vec![]
        }
    }

    pub fn from(bytes: Vec<u8>) -> Encapsulation {
        Encapsulation {
            size: 6 + bytes.len() as i32,
            major: 1,
            minor: 1,
            data: bytes
        }
    }
}