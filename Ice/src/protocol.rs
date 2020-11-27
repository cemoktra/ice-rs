use std::string::FromUtf8Error;
use std::convert::TryInto;
use std::collections::HashMap;

use crate::errors::Error;
use crate::serialization::{Serialize, Deserialize};


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

impl Encapsulation {
    pub fn new(data: &Vec<u8>) -> Encapsulation {
        Encapsulation {
            size: 6 + data.len() as i32,
            major: 1,
            minor: 1,
            data: data.clone()
        }
    }
}

#[derive(Debug)]
pub struct ReplyData {
    pub request_id: i32,
    pub status: u8,
    pub body: Encapsulation
}

impl std::convert::From<FromUtf8Error> for Error {
    fn from(_err: FromUtf8Error) -> Error {
        Error::CannotDeserialize
    }
}

impl Deserialize for Header {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() < 14 {
            return Err(Error::CannotDeserialize);
        }

        let magic = String::from_utf8(bytes[0..4].to_vec())?;
        if magic != "IceP" {
            return Err(Error::WrongProtocolMagic);
        }
        let message_size = match bytes[10..14].try_into() {
            Ok(barray) => i32::from_le_bytes(barray),
            _ => return Err(Error::CannotDeserialize)
        };
        Ok(Header {
            magic: magic,
            protocol_major: bytes[4],
            protocol_minor: bytes[5],
            encoding_major: bytes[6],
            encoding_minor: bytes[7],
            message_type: bytes[8],
            compression_status: bytes[9],
            message_size: message_size
        })
    }
}

impl Deserialize for Encapsulation {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() < 6 {
            return Err(Error::CannotDeserialize);
        }

        let size = match bytes[0..4].try_into() {
            Ok(barray) => i32::from_le_bytes(barray),
            _ => return Err(Error::CannotDeserialize)
        };

        Ok(Encapsulation {
            size: size,
            major: bytes[4],
            minor: bytes[5],
            data: bytes[6..bytes.len()].to_vec()
        })
    }
}
impl Deserialize for ReplyData {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() < 11 {
            return Err(Error::CannotDeserialize);
        }
        let request_id = match bytes[0..4].try_into() {
            Ok(barray) => i32::from_le_bytes(barray),
            _ => return Err(Error::CannotDeserialize)
        };

        Ok(ReplyData {
            request_id: request_id,
            status: bytes[4],
            body: Encapsulation::from_bytes(&bytes[5..bytes.len()])?
        })
    }
}

impl Serialize for Header {
    fn to_bytes(&self) -> Result<Vec<u8>, Error>
    {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend(self.magic.as_bytes());
        buffer.push(self.protocol_major);
        buffer.push(self.protocol_minor);
        buffer.push(self.encoding_major);
        buffer.push(self.encoding_minor);
        buffer.push(self.message_type);
        buffer.push(self.compression_status);
        buffer.extend(&self.message_size.to_le_bytes());

        Ok(buffer)
    }
}

fn serialize_size(size: i32) -> Vec<u8> {
    if size < 255 {
        vec![size as u8]
    } else {
        let mut bytes = vec![255];
        bytes.extend(&size.to_le_bytes());
        bytes
    }
}

fn serialize_string(s: &str) -> Vec<u8> {
    let mut bytes = serialize_size(s.len() as i32);
    bytes.extend(s.as_bytes());
    bytes
}

fn serialize_string_seq(seq: &Vec<String>) -> Vec<u8> {
    let mut bytes = serialize_size(seq.len() as i32);
    for item in seq {
        bytes.extend(serialize_string(item));
    }
    bytes
}

fn serialize_dict(dict: &HashMap<String, String>) -> Vec<u8> {
    let mut bytes = serialize_size(dict.len() as i32);
    for (key, value) in dict {
        bytes.extend(serialize_string(key));
        bytes.extend(serialize_string(value));
    }
    bytes
}

impl Serialize for Identity {
    fn to_bytes(&self) -> Result<Vec<u8>, Error>
    {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend(serialize_string(&self.name));
        buffer.extend(serialize_string(&self.category));
        Ok(buffer)
    }
}

impl Serialize for Encapsulation {
    fn to_bytes(&self) -> Result<Vec<u8>, Error>
    {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend(&self.size.to_le_bytes());
        buffer.push(self.major);
        buffer.push(self.minor);
        if self.data.len() > 0 {
            buffer.extend(&self.data);
        }
        Ok(buffer)
    }
}

impl Serialize for RequestData {
    fn to_bytes(&self) -> Result<Vec<u8>, Error>
    {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend(&self.request_id.to_le_bytes());
        buffer.extend(self.id.to_bytes()?);
        buffer.extend(serialize_string_seq(&self.facet));
        buffer.extend(serialize_string(&self.operation));
        buffer.push(self.mode);
        buffer.extend(serialize_dict(&self.context));
        buffer.extend(self.params.to_bytes()?);

        Ok(buffer)
    }
}