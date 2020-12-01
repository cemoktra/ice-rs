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
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error> where Self: Sized;
}

pub trait FromEncapsulation {
    type Output;
    fn from_encapsulation(body: Encapsulation) -> Result<Self::Output, Error>;
}

pub trait AsEncapsulation {
    fn as_encapsulation(&self) -> Result<Encapsulation, Error>;
}


// BASIC ENCODING FUNCTIONS

pub fn encode_size(size: i32) -> Vec<u8> {
    if size < 255 {
        vec![size as u8]
    } else {
        let mut bytes = vec![255];
        bytes.extend(encode_int(size));
        bytes
    }
}

pub fn decode_size(bytes: &[u8], read_bytes: &mut i32) -> i32 {
    if bytes[0] == 255 {
        if bytes.len() < 5 {
            0
        } else {            
            match decode_int(&bytes[1..5], read_bytes) {
                Ok(size) => {
                    *read_bytes = *read_bytes + 1;
                    size
                },
                _ => 0
            }
        }
    } else {
        if bytes.len() < 1 {
            0
        } else  {
            *read_bytes = *read_bytes + 1;
            bytes[0] as i32
        }
    }
}

pub fn encode_string(s: &str) -> Vec<u8>
{  
    let mut bytes = encode_size(s.len() as i32);
    bytes.extend(s.as_bytes());
    bytes
}

pub fn decode_string(bytes: &[u8], read_bytes: &mut i32) -> Result<String, Error>
{  
    let mut read = 0;
    let size = decode_size(bytes, &mut read);
    let s = String::from_utf8(bytes[read as usize..read as usize + size as usize].to_vec())?;
    *read_bytes = *read_bytes + read + size;
    Ok(s)
}

pub fn encode_dict(dict: &HashMap<String, String>) -> Vec<u8> {
    let mut bytes = encode_size(dict.len() as i32);
    for (key, value) in dict {
        bytes.extend(encode_string(key));
        bytes.extend(encode_string(value));
    }
    bytes
}

pub fn decode_dict(bytes: &[u8], read_bytes: &mut i32) -> Result<HashMap<String, String>, Error> {
    let mut read = 0;
    let size = decode_size(bytes, &mut read);
    let mut dict: HashMap<String, String> = HashMap::new();

    for _i in 0..size {
        let key = decode_string(&bytes[read as usize..bytes.len()], &mut read)?;
        let value = decode_string(&bytes[read as usize..bytes.len()], &mut read)?;
        dict.insert(key, value);
    }
    *read_bytes = *read_bytes + read;
    Ok(dict)
}

pub fn encode_string_seq(seq: &Vec<String>) -> Vec<u8> {
    let mut bytes = encode_size(seq.len() as i32);
    for item in seq {
        bytes.extend(encode_string(item));
    }
    bytes
}

pub fn decode_string_seq(bytes: &[u8], read_bytes: &mut i32) -> Result<Vec<String>, Error> {
    let mut read = 0;
    let size = decode_size(bytes, &mut read);
    let mut string_seq: Vec<String> = vec![];

    for _i in 0..size {
        string_seq.push(decode_string(&bytes[read as usize..bytes.len()], &mut read)?);
    }
    *read_bytes = *read_bytes + read;
    Ok(string_seq)
}

pub fn decode_byte(bytes: &[u8], read_bytes: &mut i32) -> Result<u8, Error>
{   
    *read_bytes = *read_bytes + 1;
    Ok(bytes[0])
}

pub fn encode_short(n: i16) -> Vec<u8>
{  
    n.to_le_bytes().to_vec()
}

pub fn decode_short(bytes: &[u8], read_bytes: &mut i32) -> Result<i16, Error>
{   
    if bytes.len() < 2 {
        return Err(Error::CannotDeserialize);
    }
    match bytes[0..2].try_into() {
        Ok(barray) => {
            *read_bytes = *read_bytes + 2;
            Ok(i16::from_le_bytes(barray))
        },
        _ => Err(Error::CannotDeserialize)
    }
}

pub fn encode_int(n: i32) -> Vec<u8>
{  
    n.to_le_bytes().to_vec()
}

pub fn decode_int(bytes: &[u8], read_bytes: &mut i32) -> Result<i32, Error>
{   
    if bytes.len() < 4 {
        return Err(Error::CannotDeserialize);
    }
    match bytes[0..4].try_into() {
        Ok(barray) => {
            *read_bytes = *read_bytes + 4;
            Ok(i32::from_le_bytes(barray))
        },
        _ => Err(Error::CannotDeserialize)
    }
}

pub fn encode_long(n: i64) -> Vec<u8>
{  
    n.to_le_bytes().to_vec()
}

pub fn decode_long(bytes: &[u8], read_bytes: &mut i32) -> Result<i64, Error>
{   
    if bytes.len() < 8 {
        return Err(Error::CannotDeserialize);
    }
    match bytes[0..8].try_into() {
        Ok(barray) => {
            *read_bytes = *read_bytes + 8;
            Ok(i64::from_le_bytes(barray))
        },
        _ => Err(Error::CannotDeserialize)
    }
}

pub fn encode_bool(b: bool) -> Vec<u8>
{  
    vec![if b { 1 } else { 0 }]
}

pub fn decode_bool(bytes: &[u8], read_bytes: &mut i32) -> Result<bool, Error>
{   
    if bytes.len() < 1 {
        return Err(Error::CannotDeserialize);
    }
    *read_bytes = *read_bytes + 1;
    Ok(bytes[0] != 0)
}


// implement encapsulation traits for basic types
// String
impl FromEncapsulation for String {
    type Output = Self;

    fn from_encapsulation(body: Encapsulation) -> Result<Self::Output, Error>
    {
        let mut read_bytes = 0;
        decode_string(&body.data, &mut read_bytes)
    }
}

impl AsEncapsulation for String {
    fn as_encapsulation(&self) -> Result<Encapsulation, Error> {
        let bytes = encode_string(self);
        Ok(Encapsulation {
            size: 6 + bytes.len() as i32,
            major: 1,
            minor: 1,
            data: bytes.to_vec()
        })
    }
}

impl AsEncapsulation for &str {
    fn as_encapsulation(&self) -> Result<Encapsulation, Error> {
        let bytes = encode_string(self);
        Ok(Encapsulation {
            size: 6 + bytes.len() as i32,
            major: 1,
            minor: 1,
            data: bytes.to_vec()
        })
    }
}

impl AsEncapsulation for Vec<String> {
    fn as_encapsulation(&self) -> Result<Encapsulation, Error> {
        let bytes = encode_string_seq(self);
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
        let mut read_bytes = 0;
        decode_string_seq(&body.data, &mut read_bytes)
    }
}

impl AsEncapsulation for bool {
    fn as_encapsulation(&self) -> Result<Encapsulation, Error> {
        Ok(Encapsulation {
            size: 7,
            major: 1,
            minor: 1,
            data: encode_bool(*self)
        })
    }
}

impl FromEncapsulation for bool {
    type Output = Self;

    fn from_encapsulation(body: Encapsulation) -> Result<Self::Output, Error>
    {
        let mut read_bytes = 0;
        decode_bool(&body.data, &mut read_bytes)
    }
}

impl AsEncapsulation for i16 {
    fn as_encapsulation(&self) -> Result<Encapsulation, Error> {
        let encoded = encode_short(*self);
        Ok(Encapsulation {
            size: 6 + encoded.len() as i32,
            major: 1,
            minor: 1,
            data: encoded
        })
    }
}

impl FromEncapsulation for i16 {
    type Output = Self;

    fn from_encapsulation(body: Encapsulation) -> Result<Self::Output, Error>
    {
        let mut read_bytes = 0;
        decode_short(&body.data, &mut read_bytes)
    }
}

impl AsEncapsulation for i32 {
    fn as_encapsulation(&self) -> Result<Encapsulation, Error> {
        let encoded = encode_int(*self);
        Ok(Encapsulation {
            size: 6 + encoded.len() as i32,
            major: 1,
            minor: 1,
            data: encoded
        })
    }
}

impl FromEncapsulation for i32 {
    type Output = Self;

    fn from_encapsulation(body: Encapsulation) -> Result<Self::Output, Error>
    {
        let mut read_bytes = 0;
        decode_int(&body.data, &mut read_bytes)
    }
}

impl AsEncapsulation for i64 {
    fn as_encapsulation(&self) -> Result<Encapsulation, Error> {
        let encoded = encode_long(*self);
        Ok(Encapsulation {
            size: 6 + encoded.len() as i32,
            major: 1,
            minor: 1,
            data: encoded
        })
    }
}

impl FromEncapsulation for i64 {
    type Output = Self;

    fn from_encapsulation(body: Encapsulation) -> Result<Self::Output, Error>
    {
        let mut read_bytes = 0;
        decode_long(&body.data, &mut read_bytes)
    }
}


impl AsEncapsulation for HashMap<String, String> {
    fn as_encapsulation(&self) -> Result<Encapsulation, Error> {
        let encoded = encode_dict(self);
        Ok(Encapsulation {
            size: 6 + encoded.len() as i32,
            major: 1,
            minor: 1,
            data: encoded
        })
    }
}

impl FromEncapsulation for HashMap<String, String> {
    type Output = Self;

    fn from_encapsulation(body: Encapsulation) -> Result<Self::Output, Error>
    {
        let mut read_bytes = 0;
        decode_dict(&body.data, &mut read_bytes)
    }
}


// PROTOCOL STRUCT AS/FROM BYTES
impl AsBytes for Identity {
    fn as_bytes(&self) -> Result<Vec<u8>, Error>
    {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend(encode_string(&self.name));
        buffer.extend(encode_string(&self.category));
        Ok(buffer)
    }
}

impl FromBytes for Identity {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error> {
        let mut read = 0;
        let name = decode_string(&bytes[read as usize..bytes.len()], &mut read)?;
        let category = decode_string(&bytes[read as usize..bytes.len()], &mut read)?;
        println!("identity with  {} bytes", read);
        *read_bytes = *read_bytes + read;
        Ok(Identity {
            name: name,
            category: category
        })
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

impl FromBytes for Encapsulation {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error> {
        let mut read: i32 = 0;
        if bytes.len() < 6 {
            return Err(Error::CannotDeserialize);
        }

        let size = decode_int(&bytes[read as usize..bytes.len()], &mut read)?;
        let major = decode_byte(&bytes[read as usize..bytes.len()], &mut read)?;
        let minor = decode_byte(&bytes[read as usize..bytes.len()], &mut read)?;
        *read_bytes = *read_bytes + read + (bytes.len() as i32 - read);

        Ok(Encapsulation {
            size: size,
            major: major,
            minor: minor,
            data: bytes[read as usize..bytes.len()].to_vec()
        })
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

impl FromBytes for RequestData {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error> {
        let mut read = 0;
        let request_id = decode_int(bytes, &mut read)?;
        let id = Identity::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        let facet = decode_string_seq(&bytes[read as usize..bytes.len()], &mut read)?;
        let operation = decode_string(&bytes[read as usize..bytes.len()], &mut read)?;
        let mode = decode_byte(&bytes[read as usize..bytes.len()], &mut read)?;
        let context = decode_dict(&bytes[read as usize..bytes.len()], &mut read)?;
        let encapsulation = Encapsulation::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        *read_bytes = *read_bytes + read;

        Ok(RequestData {
            request_id: request_id,
            id: id,
            facet: facet,
            operation: operation,
            mode: mode,
            context: context,
            params: encapsulation
        })
    }
}


impl AsBytes for ReplyData {
    fn as_bytes(&self) -> Result<Vec<u8>, Error>
    {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend(encode_int(self.request_id));
        buffer.push(self.status);
        buffer.extend(self.body.as_bytes()?);

        Ok(buffer)
    }
}

impl FromBytes for ReplyData {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error> {
        let mut read: i32 = 0;
        if bytes.len() < 11 {
            return Err(Error::CannotDeserialize);
        }

        let request_id = decode_int(&bytes[read as usize..bytes.len()], &mut read)?;
        let status = decode_byte(&bytes[read as usize..bytes.len()], &mut read)?;
        let encapsulation = Encapsulation::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        *read_bytes = *read_bytes + read;
        Ok(ReplyData {
            request_id: request_id,
            status: status,
            body: encapsulation
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
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error> {
        if bytes.len() < 14 {
            return Err(Error::CannotDeserialize);
        }

        let magic = String::from_utf8(bytes[0..4].to_vec())?;
        if magic != "IceP" {
            return Err(Error::WrongProtocolMagic);
        }        
        let mut read: i32 = 4;
        let protocol_major = decode_byte(&bytes[read as usize..bytes.len()], &mut read)?;
        let protocol_minor = decode_byte(&bytes[read as usize..bytes.len()], &mut read)?;
        let encoding_major = decode_byte(&bytes[read as usize..bytes.len()], &mut read)?;
        let encoding_minor = decode_byte(&bytes[read as usize..bytes.len()], &mut read)?;
        let message_type = decode_byte(&bytes[read as usize..bytes.len()], &mut read)?;
        let comression_status = decode_byte(&bytes[read as usize..bytes.len()], &mut read)?;
        let message_size = decode_int(&bytes[read as usize..bytes.len()], &mut read)?;
        *read_bytes = *read_bytes + read;

        Ok(Header {
            magic: magic,
            protocol_major: protocol_major,
            protocol_minor: protocol_minor,
            encoding_major: encoding_major,
            encoding_minor: encoding_minor,
            message_type: message_type,
            compression_status: comression_status,
            message_size: message_size
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_size_encoding() {
        let mut read_bytes = 0;
        let encoded = encode_size(10);
        let decoded = decode_size(&encoded, &mut read_bytes);
        assert_eq!(10, decoded);
        assert_eq!(1, read_bytes);

        read_bytes = 0;
        let encoded = encode_size(500);
        let decoded = decode_size(&encoded, &mut read_bytes);
        assert_eq!(500, decoded);
        assert_eq!(5, read_bytes);
    }

    #[test]
    fn test_string_encoding() {
        let mut read_bytes = 0;
        let encoded = encode_string("Hello");
        let decoded = decode_string(&encoded, &mut read_bytes).expect("Cannot decode test string");
        assert_eq!("Hello", decoded);
        assert_eq!(6, read_bytes);
    }

    #[test]
    fn test_dict_encoding() {
        let mut read_bytes = 0;
        let mut dict = HashMap::new();
        dict.insert(String::from("Hello"), String::from("World"));

        let encoded = encode_dict(&dict);
        let decoded = decode_dict(&encoded, &mut read_bytes).expect("Cannot decode test dict");
        assert!(decoded.contains_key("Hello"));
        assert_eq!("World", decoded.get("Hello").unwrap_or(&String::from("")));
    }

    #[test]
    fn test_string_seq_encoding() {
        let mut read_bytes = 0;
        let seq = vec![String::from("Hello"), String::from("World")];
        let encoded = encode_string_seq(&seq);
        let decoded = decode_string_seq(&encoded, &mut read_bytes).expect("Cannot decode test dict");
        assert_eq!(2, decoded.len());
        assert_eq!(seq, decoded);
    }

    #[test]
    fn test_short_encoding() {
        let mut read_bytes = 0;
        let value: i16 = 3;
        let encoded = encode_short(value);
        let decoded = decode_short(&encoded, &mut read_bytes).expect("Cannot decode test short");
        assert_eq!(value, decoded);
    }

    #[test]
    fn test_int_encoding() {
        let mut read_bytes = 0;
        let value: i32 = 3;
        let encoded = encode_int(value);
        let decoded = decode_int(&encoded, &mut read_bytes).expect("Cannot decode test int");
        assert_eq!(value, decoded);
    }

    #[test]
    fn test_long_encoding() {
        let mut read_bytes = 0;
        let value: i64 = 3;
        let encoded = encode_long(value);
        let decoded = decode_long(&encoded, &mut read_bytes).expect("Cannot decode test long");
        assert_eq!(value, decoded);
    }

    #[test]
    fn test_bool_encoding() {
        let mut read_bytes = 0;
        let value = true;
        let encoded = encode_bool(value);
        let decoded = decode_bool(&encoded, &mut read_bytes).expect("Cannot decode test bool");
        assert_eq!(value, decoded);
    }

    #[test]
    fn test_string_encapsulation() {
        let value = String::from("Hello World");
        let encapsulation = value.as_encapsulation().expect("Cannot encapsulate test string");
        assert_eq!(18, encapsulation.size);
        let decoded = String::from_encapsulation(encapsulation).expect("Cannot decode test string from encapsulation");
        assert_eq!(value, decoded);
    }

    #[test]
    fn test_string_seq_encapsulation() {
        let seq = vec![String::from("Hello"), String::from("World")];
        let encapsulation = seq.as_encapsulation().expect("Cannot encapsulate test string seq");
        assert_eq!(19, encapsulation.size);
        let decoded = Vec::from_encapsulation(encapsulation).expect("Cannot decode test string seq from encapsulation");
        assert_eq!(seq, decoded);
    }

    #[test]
    fn test_bool_encapsulation() {
        let value = true;
        let encapsulation = value.as_encapsulation().expect("Cannot encapsulate test bool");
        assert_eq!(7, encapsulation.size);
        let decoded = bool::from_encapsulation(encapsulation).expect("Cannot decode test bool from encapsulation");
        assert_eq!(value, decoded);
    }

    #[test]
    fn test_short_encapsulation() {
        let value: i16 = 15;
        let encapsulation = value.as_encapsulation().expect("Cannot encapsulate test short");
        assert_eq!(8, encapsulation.size);
        let decoded = i16::from_encapsulation(encapsulation).expect("Cannot decode test short from encapsulation");
        assert_eq!(value, decoded);
    }

    #[test]
    fn test_int_encapsulation() {
        let value: i32 = 123;
        let encapsulation = value.as_encapsulation().expect("Cannot encapsulate test int");
        assert_eq!(10, encapsulation.size);
        let decoded = i32::from_encapsulation(encapsulation).expect("Cannot decode test int from encapsulation");
        assert_eq!(value, decoded);
    }

    #[test]
    fn test_long_encapsulation() {
        let value: i64 = 12345678;
        let encapsulation = value.as_encapsulation().expect("Cannot encapsulate test long");
        assert_eq!(14, encapsulation.size);
        let decoded = i64::from_encapsulation(encapsulation).expect("Cannot decode test long from encapsulation");
        assert_eq!(value, decoded);
    }

    #[test]
    fn test_dict_encapsulation() {
        let mut dict = HashMap::new();
        dict.insert(String::from("Hello"), String::from("World"));

        let encapsulation = dict.as_encapsulation().expect("Cannot encapsulate test dict");
        assert_eq!(19, encapsulation.size);
        let decoded = HashMap::from_encapsulation(encapsulation).expect("Cannot decode test dict from encapsulation");
        assert_eq!(dict, decoded);
    }

    #[test]
    fn test_identity_ecoding() {
        let mut read_bytes = 0;
        let id = Identity {
            name: String::from("Hello"),
            category: String::from(""),
        };
        let bytes = id.as_bytes().expect("Cannot encode test identity");
        let decoded = Identity::from_bytes(&bytes, &mut read_bytes).expect("Cannot decode test identity");
        assert_eq!(7, read_bytes);
        assert_eq!(id.name, decoded.name);
        assert_eq!(id.category, decoded.category);
    }

    #[test]
    fn test_header_ecoding() {
        let mut read_bytes = 0;
        let header = Header::new(0, 14);
        let bytes = header.as_bytes().expect("Cannot encode test header");
        let decoded = Header::from_bytes(&bytes, &mut read_bytes).expect("Cannot decode test header");
        assert_eq!(14, read_bytes);
        assert_eq!(header.magic, decoded.magic);
        assert_eq!(header.message_size, decoded.message_size);
        assert_eq!(header.message_type, decoded.message_type);
        assert_eq!(header.magic, decoded.magic);
    }

    #[test]
    fn test_request_ecoding() {
        let mut read_bytes = 0;
        let request = RequestData {
            request_id: 1,
            id: Identity {
                name: String::from("Test"),
                category: String::from(""),
            },
            facet: vec![],
            operation: String::from("Op"),
            mode: 0,
            context: HashMap::new(),
            params: Encapsulation::empty()
        };
        let bytes = request.as_bytes().expect("Cannot encode test request");
        let decoded = RequestData::from_bytes(&bytes, &mut read_bytes).expect("Cannot decode test request");
        assert_eq!(22, read_bytes);
        assert_eq!(request.request_id, decoded.request_id);
        assert_eq!(request.id.name, decoded.id.name);
        assert_eq!(request.facet, decoded.facet);
        assert_eq!(request.operation, decoded.operation);
        assert_eq!(request.mode, decoded.mode);
        assert_eq!(request.context, decoded.context);
    }

    #[test]
    fn test_reply_encoding() {
        let mut read_bytes = 0;
        let reply = ReplyData {
            request_id: 1,
            status: 0,
            body: Encapsulation::empty()
        };
        let bytes = reply.as_bytes().expect("Cannot encode test reply");
        let decoded = ReplyData::from_bytes(&bytes, &mut read_bytes).expect("Cannot decode test reply");        
        assert_eq!(11, read_bytes);
        assert_eq!(reply.request_id, decoded.request_id);
        assert_eq!(reply.status, decoded.status);
    }
}