use crate::protocol::{Encapsulation, Header, Identity, RequestData, ReplyData};
use crate::errors::Error;

use std::convert::TryInto;
use std::collections::HashMap;
use std::string::FromUtf8Error;


impl std::convert::From<FromUtf8Error> for Error {
    fn from(_err: FromUtf8Error) -> Error {
        Error::CannotDeserialize
    }
}

// TRAITS
pub trait AsBytes {
    fn as_bytes(&self) -> Result<Vec<u8>, Error>;
}

pub trait FromBytes {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Error> where Self: Sized;
}

pub trait FromEncapsulation {
    type Output;
    fn from_encapsulation(body: Encapsulation) -> Result<Self::Output, Error>;
}

pub trait AsEncapsulation {
    fn as_encapsulation(self) -> Result<Encapsulation, Error>;
}


// BASIC ENCODING FUNCTIONS

fn encode_size(size: i32) -> Vec<u8> {
    if size < 255 {
        vec![size as u8]
    } else {
        let mut bytes = vec![255];
        bytes.extend(&size.to_le_bytes());
        bytes
    }
}

pub fn decode_size(bytes: &[u8]) -> (i32, u8) {
    if bytes[0] == 255 {
        if bytes.len() < 5 {
            (0, 0)
        } else {
            match bytes[0..4].try_into() {
                Ok(barray) => (i32::from_le_bytes(barray), 5),
                _ => (0, 0)
            }
        }
    } else {
        (bytes[0] as i32, 1)
    }
}

fn encode_string(s: &str) -> Vec<u8>
{  
    let mut bytes = encode_size(s.len() as i32);
    bytes.extend(s.as_bytes());
    bytes
}

fn decode_string(bytes: &[u8]) -> Result<(String, i32), Error>
{  
    let (size, read) = decode_size(bytes);
    let s = String::from_utf8(bytes[read as usize..read as usize + size as usize].to_vec())?;
    Ok((s, read as i32 + size))
}

fn encode_dict(dict: &HashMap<String, String>) -> Vec<u8> {
    let mut bytes = encode_size(dict.len() as i32);
    for (key, value) in dict {
        bytes.extend(encode_string(key));
        bytes.extend(encode_string(value));
    }
    bytes
}

fn encode_string_seq(seq: &Vec<String>) -> Vec<u8> {
    let mut bytes = encode_size(seq.len() as i32);
    for item in seq {
        bytes.extend(encode_string(item));
    }
    bytes
}

fn decode_string_seq(bytes: &[u8]) -> Result<Vec<String>, Error> {
    let (size, read) = decode_size(bytes);
    let mut string_seq: Vec<String> = vec![];
    let mut current_position = read as i32;

    for _i in 0..size {
        let (s, read) = decode_string(&bytes[current_position as usize..bytes.len()])?;
        string_seq.push(s);
        current_position = current_position + read;
    }

    Ok(string_seq)
}

pub fn encode_short(n: i16) -> Vec<u8>
{  
    n.to_le_bytes().to_vec()
}

pub fn decode_short(bytes: &[u8]) -> Result<i16, Error>
{   
    if bytes.len() < 2 {
        return Err(Error::CannotDeserialize);
    }
    match bytes[0..2].try_into() {
        Ok(barray) => Ok(i16::from_le_bytes(barray)),
        _ => Err(Error::CannotDeserialize)
    }
}

pub fn encode_int(n: i32) -> Vec<u8>
{  
    n.to_le_bytes().to_vec()
}

pub fn decode_int(bytes: &[u8]) -> Result<i32, Error>
{   
    if bytes.len() < 4 {
        return Err(Error::CannotDeserialize);
    }
    match bytes[0..4].try_into() {
        Ok(barray) => Ok(i32::from_le_bytes(barray)),
        _ => Err(Error::CannotDeserialize)
    }
}

pub fn encode_long(n: i64) -> Vec<u8>
{  
    n.to_le_bytes().to_vec()
}

pub fn decode_long(bytes: &[u8]) -> Result<i64, Error>
{   
    if bytes.len() < 8 {
        return Err(Error::CannotDeserialize);
    }
    match bytes[0..8].try_into() {
        Ok(barray) => Ok(i64::from_le_bytes(barray)),
        _ => Err(Error::CannotDeserialize)
    }
}


// implement encapsulation traits for basic types
// String
impl FromEncapsulation for String {
    type Output = Self;

    fn from_encapsulation(body: Encapsulation) -> Result<Self::Output, Error>
    {
        let (s, _) = decode_string(&body.data)?;
        Ok(s)
    }
}

impl AsEncapsulation for String {
    fn as_encapsulation(self) -> Result<Encapsulation, Error> {
        let bytes = self.as_bytes();
        Ok(Encapsulation {
            size: 6 + bytes.len() as i32,
            major: 1,
            minor: 1,
            data: bytes.to_vec()
        })
    }
}

impl AsEncapsulation for &str {
    fn as_encapsulation(self) -> Result<Encapsulation, Error> {
        let bytes = encode_string(self);
        Ok(Encapsulation {
            size: 6 + bytes.len() as i32,
            major: 1,
            minor: 1,
            data: bytes.to_vec()
        })
    }
}



impl FromEncapsulation for Vec<String> {
    type Output = Self;

    fn from_encapsulation(body: Encapsulation) -> Result<Self::Output, Error>
    {
        Ok(decode_string_seq(&body.data)?)
    }
}

impl FromEncapsulation for bool {
    type Output = Self;

    fn from_encapsulation(body: Encapsulation) -> Result<Self::Output, Error>
    {
        if body.data.len() == 1 {
            Ok(body.data[0] != 0)
        } else {
            Ok(false)
        }
    }
}





// 
impl AsBytes for Identity {
    fn as_bytes(&self) -> Result<Vec<u8>, Error>
    {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend(encode_string(&self.name));
        buffer.extend(encode_string(&self.category));
        Ok(buffer)
    }
}

impl AsBytes for Encapsulation {
    fn as_bytes(&self) -> Result<Vec<u8>, Error>
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

impl AsBytes for RequestData {
    fn as_bytes(&self) -> Result<Vec<u8>, Error>
    {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend(&self.request_id.to_le_bytes());
        buffer.extend(self.id.as_bytes()?);
        buffer.extend(encode_string_seq(&self.facet));
        buffer.extend(encode_string(&self.operation));
        buffer.push(self.mode);
        buffer.extend(encode_dict(&self.context));
        buffer.extend(self.params.as_bytes()?);

        Ok(buffer)
    }
}

impl FromBytes for Encapsulation {
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

impl FromBytes for ReplyData {
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

impl AsBytes for Header {
    fn as_bytes(&self) -> Result<Vec<u8>, Error>
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

impl FromBytes for Header {
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

