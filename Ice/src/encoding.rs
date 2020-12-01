use crate::protocol::{Encapsulation, Header, Identity, RequestData, ReplyData};
use crate::errors::Error;

use std::convert::TryInto;
use std::collections::HashMap;
use std::string::FromUtf8Error;
use std::hash::Hash;

impl std::convert::From<FromUtf8Error> for Error {
    fn from(_err: FromUtf8Error) -> Error {
        Error::DecodingError
    }
}

pub struct IceSize {
    pub size: i32
}

// TRAITS
pub trait ToBytes {
    fn to_bytes(&self) -> Result<Vec<u8>, Error>;
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
impl ToBytes for IceSize {
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        if self.size < 255 {
            Ok(vec![self.size as u8])
        } else {
            let mut bytes = vec![255];
            bytes.extend(self.size.to_bytes()?);
            Ok(bytes)
        }    
    }
}

impl FromBytes for IceSize {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error>
    where Self: Sized {
        if bytes.len() < 1 {
            Err(Error::DecodingError)
        }   
        else if bytes[0] == 255 {
            if bytes.len() < 5 {
                Err(Error::DecodingError)
            } else {
                *read_bytes = 1;
                Ok(IceSize {
                    size: i32::from_bytes(&bytes[1..5], read_bytes)?
                })
            }
        } else {
            Ok(IceSize {
                size: u8::from_bytes(bytes, read_bytes)? as i32
            })
        }   
    }
}

impl ToBytes for str {
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes = IceSize{size: self.len() as i32}.to_bytes()?;
        bytes.extend(self.as_bytes());
        Ok(bytes)
    }
}

impl ToBytes for String {
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes = IceSize{size: self.len() as i32}.to_bytes()?;
        bytes.extend(self.as_bytes());
        Ok(bytes)
    }
}

impl FromBytes for String {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error>
    where Self: Sized {
        let mut read = 0;
        let size = IceSize::from_bytes(bytes, &mut read)?.size;
        let s = String::from_utf8(bytes[read as usize..read as usize + size as usize].to_vec())?;
        *read_bytes = *read_bytes + read + size;
        Ok(s)
    }
}


impl<T: ToBytes, U: ToBytes> ToBytes for HashMap<T, U> {
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes = IceSize{size: self.len() as i32}.to_bytes()?;
        for (key, value) in self {
            bytes.extend(key.to_bytes()?);
            bytes.extend(value.to_bytes()?);
        }
        Ok(bytes)
    }
}

impl<T: FromBytes + Eq + Hash, U: FromBytes> FromBytes for HashMap<T, U> {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error>
    where Self: Sized {
        let mut read = 0;
        let size = IceSize::from_bytes(bytes, &mut read)?.size;
        let mut dict: HashMap<T, U> = HashMap::new();

        for _i in 0..size {
            let key = T::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
            let value = U::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
            dict.insert(key, value);
        }
        *read_bytes = *read_bytes + read;
        Ok(dict)
    }
}

impl<T: ToBytes> ToBytes for Vec<T>
{
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes = IceSize{size: self.len() as i32}.to_bytes()?;
        for item in self {
            bytes.extend(item.to_bytes()?);
        }
        Ok(bytes)
    }
}

impl<T: FromBytes> FromBytes for Vec<T>
{
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error>
    where Self: Sized {
        let mut read = 0;
        let size = IceSize::from_bytes(bytes, &mut read)?.size;
        let mut seq: Vec<T> = vec![];

        for _i in 0..size {
            seq.push(T::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?);
        }
        *read_bytes = *read_bytes + read;
        Ok(seq)
    }
}

impl ToBytes for u8 {
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        Ok(vec![*self])
    }
}

impl FromBytes for u8 {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error>
    where Self: Sized {
        *read_bytes = *read_bytes + 1;
        Ok(bytes[0])
    }
}

impl ToBytes for i16 {
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        Ok(self.to_le_bytes().to_vec())
    }
}

impl FromBytes for i16 {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error>
    where Self: Sized {
        let size = std::mem::size_of::<i16>();
        if bytes.len() < size {
            return Err(Error::DecodingError);
        }
        match bytes[0..size].try_into() {
            Ok(barray) => {
                *read_bytes = *read_bytes + size as i32;
                Ok(i16::from_le_bytes(barray))
            },
            _ => Err(Error::DecodingError)
        }
    }
}

impl ToBytes for i32 {
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        Ok(self.to_le_bytes().to_vec())
    }
}

impl FromBytes for i32 {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error>
    where Self: Sized {
        let size = std::mem::size_of::<i32>();
        if bytes.len() < size {
            return Err(Error::DecodingError);
        }
        match bytes[0..size].try_into() {
            Ok(barray) => {
                *read_bytes = *read_bytes + size as i32;
                Ok(i32::from_le_bytes(barray))
            },
            _ => Err(Error::DecodingError)
        }
    }
}

impl ToBytes for i64 {
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        Ok(self.to_le_bytes().to_vec())
    }
}

impl FromBytes for i64 {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error>
    where Self: Sized {
        let size = std::mem::size_of::<i64>();
        if bytes.len() < size {
            return Err(Error::DecodingError);
        }
        match bytes[0..size].try_into() {
            Ok(barray) => {
                *read_bytes = *read_bytes + size as i32;
                Ok(i64::from_le_bytes(barray))
            },
            _ => Err(Error::DecodingError)
        }
    }
}

impl ToBytes for f32 {
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        Ok(self.to_le_bytes().to_vec())
    }
}

impl FromBytes for f32 {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error>
    where Self: Sized {
        let size = std::mem::size_of::<f32>();
        if bytes.len() < size {
            return Err(Error::DecodingError);
        }
        match bytes[0..size].try_into() {
            Ok(barray) => {
                *read_bytes = *read_bytes + size as i32;
                Ok(f32::from_le_bytes(barray))
            },
            _ => Err(Error::DecodingError)
        }
    }
}

impl ToBytes for f64 {
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        Ok(self.to_le_bytes().to_vec())
    }
}

impl FromBytes for f64 {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error>
    where Self: Sized {
        let size = std::mem::size_of::<f64>();
        if bytes.len() < size {
            return Err(Error::DecodingError);
        }
        match bytes[0..size].try_into() {
            Ok(barray) => {
                *read_bytes = *read_bytes + size as i32;
                Ok(f64::from_le_bytes(barray))
            },
            _ => Err(Error::DecodingError)
        }
    }
}

impl ToBytes for bool {
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        Ok(vec![if *self { 1 } else { 0 }])
    }
}

impl FromBytes for bool {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error>
    where Self: Sized {
        if bytes.len() < 1 {
            return Err(Error::DecodingError);
        }
        *read_bytes = *read_bytes + 1;
        Ok(bytes[0] != 0)
    }
}


// implement encapsulation traits for basic types
// String
impl FromEncapsulation for String {
    type Output = Self;

    fn from_encapsulation(body: Encapsulation) -> Result<Self::Output, Error>
    {
        let mut read_bytes = 0;
        String::from_bytes(&body.data, &mut read_bytes)
    }
}

impl AsEncapsulation for String {
    fn as_encapsulation(&self) -> Result<Encapsulation, Error> {
        let bytes = self.to_bytes()?;
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
        let bytes = self.to_bytes()?;
        Ok(Encapsulation {
            size: 6 + bytes.len() as i32,
            major: 1,
            minor: 1,
            data: bytes.to_vec()
        })
    }
}

impl<T: ToBytes> AsEncapsulation for Vec<T> {
    fn as_encapsulation(&self) -> Result<Encapsulation, Error> {
        let bytes = self.to_bytes()?;
        Ok(Encapsulation {
            size: 6 + bytes.len() as i32,
            major: 1,
            minor: 1,
            data: bytes.to_vec()
        })
    }
}

impl<T: FromBytes> FromEncapsulation for Vec<T> {
    type Output = Self;

    fn from_encapsulation(body: Encapsulation) -> Result<Self::Output, Error>
    {
        let mut read_bytes = 0;
        Vec::from_bytes(&body.data, &mut read_bytes)
    }
}

impl AsEncapsulation for bool {
    fn as_encapsulation(&self) -> Result<Encapsulation, Error> {
        Ok(Encapsulation {
            size: 7,
            major: 1,
            minor: 1,
            data: self.to_bytes()?
        })
    }
}

impl FromEncapsulation for bool {
    type Output = Self;

    fn from_encapsulation(body: Encapsulation) -> Result<Self::Output, Error>
    {
        let mut read_bytes = 0;
        bool::from_bytes(&body.data, &mut read_bytes)
    }
}

impl AsEncapsulation for i16 {
    fn as_encapsulation(&self) -> Result<Encapsulation, Error> {
        let encoded = self.to_bytes()?;
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
        i16::from_bytes(&body.data, &mut read_bytes)
    }
}

impl AsEncapsulation for i32 {
    fn as_encapsulation(&self) -> Result<Encapsulation, Error> {
        let encoded = self.to_bytes()?;
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
        i32::from_bytes(&body.data, &mut read_bytes)
    }
}

impl AsEncapsulation for i64 {
    fn as_encapsulation(&self) -> Result<Encapsulation, Error> {
        let encoded = self.to_bytes()?;
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
        i64::from_bytes(&body.data, &mut read_bytes)
    }
}

impl AsEncapsulation for f32 {
    fn as_encapsulation(&self) -> Result<Encapsulation, Error> {
        let encoded = self.to_bytes()?;
        Ok(Encapsulation {
            size: 6 + encoded.len() as i32,
            major: 1,
            minor: 1,
            data: encoded
        })
    }
}

impl FromEncapsulation for f32 {
    type Output = Self;

    fn from_encapsulation(body: Encapsulation) -> Result<Self::Output, Error>
    {
        let mut read_bytes = 0;
        f32::from_bytes(&body.data, &mut read_bytes)
    }
}

impl AsEncapsulation for f64 {
    fn as_encapsulation(&self) -> Result<Encapsulation, Error> {
        let encoded = self.to_bytes()?;
        Ok(Encapsulation {
            size: 6 + encoded.len() as i32,
            major: 1,
            minor: 1,
            data: encoded
        })
    }
}

impl FromEncapsulation for f64 {
    type Output = Self;

    fn from_encapsulation(body: Encapsulation) -> Result<Self::Output, Error>
    {
        let mut read_bytes = 0;
        f64::from_bytes(&body.data, &mut read_bytes)
    }
}

impl<T: ToBytes, U: ToBytes> AsEncapsulation for HashMap<T, U> {
    fn as_encapsulation(&self) -> Result<Encapsulation, Error> {
        let encoded = self.to_bytes()?;
        Ok(Encapsulation {
            size: 6 + encoded.len() as i32,
            major: 1,
            minor: 1,
            data: encoded
        })
    }
}

impl<T: FromBytes + Eq + Hash, U: FromBytes> FromEncapsulation for HashMap<T, U> {
    type Output = Self;

    fn from_encapsulation(body: Encapsulation) -> Result<Self::Output, Error>
    {
        let mut read_bytes = 0;
        HashMap::from_bytes(&body.data, &mut read_bytes)
    }
}


// PROTOCOL STRUCT AS/FROM BYTES
impl ToBytes for Identity {
    fn to_bytes(&self) -> Result<Vec<u8>, Error>
    {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend(self.name.to_bytes()?);
        buffer.extend(self.category.to_bytes()?);
        Ok(buffer)
    }
}

impl FromBytes for Identity {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error> {
        let mut read = 0;
        let name = String::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        let category = String::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        *read_bytes = *read_bytes + read;
        Ok(Identity {
            name: name,
            category: category
        })
    }
}

impl ToBytes for Encapsulation {
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

impl FromBytes for Encapsulation {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error> {
        let mut read: i32 = 0;
        if bytes.len() < 6 {
            return Err(Error::DecodingError);
        }

        let size = i32::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        let major = u8::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        let minor = u8::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        *read_bytes = *read_bytes + read + (bytes.len() as i32 - read);

        Ok(Encapsulation {
            size: size,
            major: major,
            minor: minor,
            data: bytes[read as usize..bytes.len()].to_vec()
        })
    }
}

impl ToBytes for RequestData {
    fn to_bytes(&self) -> Result<Vec<u8>, Error>
    {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend(self.request_id.to_bytes()?);
        buffer.extend(self.id.to_bytes()?);
        buffer.extend(self.facet.to_bytes()?);
        buffer.extend(self.operation.to_bytes()?);
        buffer.extend(self.mode.to_bytes()?);
        buffer.extend(self.context.to_bytes()?);
        buffer.extend(self.params.to_bytes()?);

        Ok(buffer)        

    }
}

impl FromBytes for RequestData {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error> {
        let mut read = 0;
        let request_id = i32::from_bytes(bytes, &mut read)?;
        let id = Identity::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        let facet = Vec::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        let operation = String::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        let mode = u8::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        let context = HashMap::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
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


impl ToBytes for ReplyData {
    fn to_bytes(&self) -> Result<Vec<u8>, Error>
    {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend(self.request_id.to_bytes()?);
        buffer.extend(self.status.to_bytes()?);
        buffer.extend(self.body.to_bytes()?);

        Ok(buffer)
    }
}

impl FromBytes for ReplyData {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error> {
        let mut read: i32 = 0;
        if bytes.len() < 11 {
            return Err(Error::DecodingError);
        }

        let request_id = i32::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        let status = u8::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        let encapsulation = Encapsulation::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        *read_bytes = *read_bytes + read;
        Ok(ReplyData {
            request_id: request_id,
            status: status,
            body: encapsulation
        })
    }
}

impl ToBytes for Header {
    fn to_bytes(&self) -> Result<Vec<u8>, Error>
    {
        let mut buffer: Vec<u8> = Vec::new();
        buffer.extend(self.magic.as_bytes());
        buffer.extend(self.protocol_major.to_bytes()?);
        buffer.extend(self.protocol_minor.to_bytes()?);
        buffer.extend(self.encoding_major.to_bytes()?);
        buffer.extend(self.encoding_minor.to_bytes()?);
        buffer.extend(self.message_type.to_bytes()?);
        buffer.extend(self.compression_status.to_bytes()?);
        buffer.extend(&self.message_size.to_le_bytes());

        Ok(buffer)
    }
}

impl FromBytes for Header {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error> {
        if bytes.len() < 14 {
            return Err(Error::DecodingError);
        }

        let magic = String::from_utf8(bytes[0..4].to_vec())?;
        if magic != "IceP" {
            return Err(Error::ProtocolError);
        }        
        let mut read: i32 = 4;
        let protocol_major = u8::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        let protocol_minor = u8::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        let encoding_major = u8::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        let encoding_minor = u8::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        let message_type = u8::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        let comression_status = u8::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        let message_size = i32::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
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
        let encoded = IceSize{size: 10}.to_bytes().expect("Could not encode size");
        let decoded = IceSize::from_bytes(&encoded, &mut read_bytes).expect("Could not decode size").size;
        assert_eq!(10, decoded);
        assert_eq!(1, read_bytes);

        read_bytes = 0;
        let encoded = IceSize{size: 500}.to_bytes().expect("Could not encode size");
        let decoded = IceSize::from_bytes(&encoded, &mut read_bytes).expect("Could not decode size").size;
        assert_eq!(500, decoded);
        assert_eq!(5, read_bytes);
    }

    #[test]
    fn test_string_encoding() {
        let mut read_bytes = 0;
        let encoded = "Hello".to_bytes().expect("Cannot necode test string");
        let decoded = String::from_bytes(&encoded, &mut read_bytes).expect("Cannot decode test string");
        assert_eq!("Hello", decoded);
        assert_eq!(6, read_bytes);
    }

    #[test]
    fn test_dict_encoding() {
        let mut read_bytes = 0;
        let mut dict = HashMap::new();
        dict.insert(String::from("Hello"), String::from("World"));

        let encoded = dict.to_bytes().expect("Cannot encode test dict");
        let decoded: HashMap<String, String> = HashMap::from_bytes(&encoded, &mut read_bytes).expect("Cannot decode test dict");
        assert!(decoded.contains_key("Hello"));
        assert_eq!("World", decoded.get("Hello").unwrap_or(&String::from("")));
    }

    #[test]
    fn test_string_seq_encoding() {
        let mut read_bytes = 0;
        let seq = vec![String::from("Hello"), String::from("World")];
        let encoded = seq.to_bytes().expect("Cannot encode test dict");
        let decoded: Vec<String> = Vec::from_bytes(&encoded, &mut read_bytes).expect("Cannot decode test dict");
        assert_eq!(2, decoded.len());
        assert_eq!(seq, decoded);
    }

    #[test]
    fn test_short_encoding() {
        let mut read_bytes = 0;
        let value: i16 = 3;
        let encoded = value.to_bytes().expect("Cannot encode test short");
        let decoded = i16::from_bytes(&encoded, &mut read_bytes).expect("Cannot decode test short");
        assert_eq!(value, decoded);
    }

    #[test]
    fn test_int_encoding() {
        let mut read_bytes = 0;
        let value: i32 = 3;
        let encoded = value.to_bytes().expect("Cannot encode test int");
        let decoded = i32::from_bytes(&encoded, &mut read_bytes).expect("Cannot decode test int");
        assert_eq!(value, decoded);
    }

    #[test]
    fn test_long_encoding() {
        let mut read_bytes = 0;
        let value: i64 = 3;
        let encoded = value.to_bytes().expect("Cannot encode test long");
        let decoded = i64::from_bytes(&encoded, &mut read_bytes).expect("Cannot decode test long");
        assert_eq!(value, decoded);
    }

    #[test]
    fn test_float_encoding() {
        let mut read_bytes = 0;
        let value: f32 = 3.14;
        let encoded = value.to_bytes().expect("Cannot encode test float");
        let decoded = f32::from_bytes(&encoded, &mut read_bytes).expect("Cannot decode test float");
        assert_eq!(value, decoded);
    }

    #[test]
    fn test_double_encoding() {
        let mut read_bytes = 0;
        let value: f64 = 3.14;
        let encoded = value.to_bytes().expect("Cannot encode test double");
        let decoded = f64::from_bytes(&encoded, &mut read_bytes).expect("Cannot decode double long");
        assert_eq!(value, decoded);
    }

    #[test]
    fn test_bool_encoding() {
        let mut read_bytes = 0;
        let value = true;
        let encoded = value.to_bytes().expect("Cannot encode test bool");
        let decoded = bool::from_bytes(&encoded, &mut read_bytes).expect("Cannot decode test bool");
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
        let decoded: Vec<String> = Vec::from_encapsulation(encapsulation).expect("Cannot decode test string seq from encapsulation");
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
    fn test_float_encapsulation() {
        let value: f32 = 3.14;
        let encapsulation = value.as_encapsulation().expect("Cannot encapsulate test float");
        assert_eq!(10, encapsulation.size);
        let decoded = f32::from_encapsulation(encapsulation).expect("Cannot decode test float from encapsulation");
        assert_eq!(value, decoded);
    }

    #[test]
    fn test_double_encapsulation() {
        let value: f64 = 3.14;
        let encapsulation = value.as_encapsulation().expect("Cannot encapsulate test double");
        assert_eq!(14, encapsulation.size);
        let decoded = f64::from_encapsulation(encapsulation).expect("Cannot decode test double from encapsulation");
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
        let bytes = id.to_bytes().expect("Cannot encode test identity");
        let decoded = Identity::from_bytes(&bytes, &mut read_bytes).expect("Cannot decode test identity");
        assert_eq!(7, read_bytes);
        assert_eq!(id.name, decoded.name);
        assert_eq!(id.category, decoded.category);
    }

    #[test]
    fn test_header_ecoding() {
        let mut read_bytes = 0;
        let header = Header::new(0, 14);
        let bytes = header.to_bytes().expect("Cannot encode test header");
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
        let bytes = request.to_bytes().expect("Cannot encode test request");
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
        let bytes = reply.to_bytes().expect("Cannot encode test reply");
        let decoded = ReplyData::from_bytes(&bytes, &mut read_bytes).expect("Cannot decode test reply");        
        assert_eq!(11, read_bytes);
        assert_eq!(reply.request_id, decoded.request_id);
        assert_eq!(reply.status, decoded.status);
    }
}