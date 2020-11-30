use ice_rs::errors::Error;
use ice_rs::encoding::{
    AsBytes, FromBytes, AsEncapsulation, FromEncapsulation,
    encode_long, decode_long, decode_size
};
use ice_rs::protocol::Encapsulation;



#[derive(Debug, Copy, Clone)]
#[repr(i8)] // based on max i8, i16, i32
pub enum RectType {
    Rect,
    Square
}

#[derive(Debug, Copy, Clone)]
pub struct Rect {
    pub left: i64,
    pub right: i64,
    pub top: i64,
    pub bottom: i64
}

#[derive(Debug, Copy, Clone)]
pub struct RectProps {
    pub width: i64,
    pub height: i64,
    pub rect_type: RectType
}


pub trait Hello {
    // base ice
    fn ice_ping(&mut self) -> Result<(), Error>;
    fn ice_is_a(&mut self) -> Result<bool, Error>;
    fn ice_id(&mut self) -> Result<String, Error>;
    fn ice_ids(&mut self) -> Result<Vec<String>, Error>;
    // hello interface
    fn say_hello(&mut self) -> Result<(), Error>;
    fn say(&mut self, text: &str) -> Result<(), Error>;
    fn calc_rect(&mut self, rc: Rect) -> Result<RectProps, Error>;
}

impl RectType {
    fn from(n: i32) -> Result<RectType, Error> {
        match n {
            0 => Ok(RectType::Rect),
            1 => Ok(RectType::Square),
            _ => Err(Error::CannotDeserialize)
        }
    }
}

impl AsBytes for Rect {
    fn as_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes = encode_long(self.left);
        bytes.extend(encode_long(self.right));
        bytes.extend(encode_long(self.top));
        bytes.extend(encode_long(self.bottom));
        Ok(bytes)
    }
}

impl FromBytes for RectProps {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let width = decode_long(&bytes[0..8])?;
        let height = decode_long(&bytes[8..16])?;
        let (enum_value, _) = decode_size(&bytes[16..bytes.len()]);

        Ok(RectProps{
            width: width,
            height: height,
            rect_type: RectType::from(enum_value)?
        })
    }
}

impl AsEncapsulation for Rect {
    fn as_encapsulation(self) -> Result<Encapsulation, Error> {
        let bytes = self.as_bytes()?;
        Ok(Encapsulation {
            size: 6 + bytes.len() as i32,
            major: 1,
            minor: 1,
            data: bytes.to_vec()
        })
    }
}

impl FromEncapsulation for RectProps {
    type Output = RectProps;

    fn from_encapsulation(body: Encapsulation) -> Result<Self::Output, Error> {
        RectProps::from_bytes(&body.data)
    }
}