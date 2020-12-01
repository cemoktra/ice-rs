use std::collections::HashMap;



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

#[derive(Debug)]
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

impl Header {
    pub fn new(message_type: u8, message_size: i32) -> Header {
        Header {
            magic: String::from("IceP"),
            protocol_major: 1,
            protocol_minor: 1,
            encoding_major: 1,
            encoding_minor: 1,
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
}