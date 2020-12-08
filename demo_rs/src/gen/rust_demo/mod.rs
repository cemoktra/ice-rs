// This file has been generated.

use ice_rs::proxy::Proxy;
use ice_rs::encoding::IceSize;
use std::convert::TryFrom;
use ice_rs::iceobject::IceObject;
use ice_rs::protocol::{Encapsulation, ReplyData};
use ice_rs::errors::Error;
use num_enum::TryFromPrimitive;
use ice_rs::encoding::{
   ToBytes, FromBytes, AsEncapsulation, FromEncapsulation
};

#[derive(Debug, Copy, Clone, TryFromPrimitive, PartialEq)]
#[repr(i32)]
pub enum RectType {
    Rect = 0,
    Square = 1,
}

impl ToBytes for RectType {
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes = Vec::new();
        bytes.extend(IceSize{size: *self as i32}.to_bytes()?);
        Ok(bytes)
    }
}

impl FromBytes for RectType {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error> {
        let mut read = 0;
        let enum_value =  IceSize::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?.size;
        *read_bytes = *read_bytes + read;

        match RectType::try_from(enum_value) {
            Ok(enum_type) => Ok(enum_type),
            _ => Err(Error::DecodingError)
        }
    }
}

impl AsEncapsulation for RectType {
    fn as_encapsulation(&self) -> Result<Encapsulation, Error> {
        let bytes = self.to_bytes()?;
        Ok(Encapsulation {
            size: 6 + bytes.len() as i32,
            major: 1,
            minor: 1,
            data: bytes.to_vec(),
        })
    }
}

impl FromEncapsulation for RectType {
    type Output = Self;

    fn from_encapsulation(body: Encapsulation) -> Result<Self::Output, Error> {
        let mut read_bytes = 0;
        Self::from_bytes(&body.data, &mut read_bytes)
    }
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rect {
    pub left: i64,
    pub right: i64,
    pub top: i64,
    pub bottom: i64,
}

impl ToBytes for Rect {
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes = Vec::new();
        bytes.extend(self.left.to_bytes()?);
        bytes.extend(self.right.to_bytes()?);
        bytes.extend(self.top.to_bytes()?);
        bytes.extend(self.bottom.to_bytes()?);
        Ok(bytes)
    }
}

impl FromBytes for Rect {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error> {
        let mut read = 0;
        let obj = Self{
            left:  i64::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?,
            right:  i64::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?,
            top:  i64::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?,
            bottom:  i64::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?,
        };
        *read_bytes = *read_bytes + read;
        Ok(obj)
    }
}

impl AsEncapsulation for Rect {
    fn as_encapsulation(&self) -> Result<Encapsulation, Error> {
        let bytes = self.to_bytes()?;
        Ok(Encapsulation {
            size: 6 + bytes.len() as i32,
            major: 1,
            minor: 1,
            data: bytes.to_vec(),
        })
    }
}

impl FromEncapsulation for Rect {
    type Output = Self;

    fn from_encapsulation(body: Encapsulation) -> Result<Self::Output, Error> {
        let mut read_bytes = 0;
        Self::from_bytes(&body.data, &mut read_bytes)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RectProps {
    pub width: i64,
    pub height: i64,
    pub rect_type: RectType,
}

impl ToBytes for RectProps {
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes = Vec::new();
        bytes.extend(self.width.to_bytes()?);
        bytes.extend(self.height.to_bytes()?);
        bytes.extend(self.rect_type.to_bytes()?);
        Ok(bytes)
    }
}

impl FromBytes for RectProps {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error> {
        let mut read = 0;
        let obj = Self{
            width:  i64::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?,
            height:  i64::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?,
            rect_type:  RectType::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?,
        };
        *read_bytes = *read_bytes + read;
        Ok(obj)
    }
}

impl AsEncapsulation for RectProps {
    fn as_encapsulation(&self) -> Result<Encapsulation, Error> {
        let bytes = self.to_bytes()?;
        Ok(Encapsulation {
            size: 6 + bytes.len() as i32,
            major: 1,
            minor: 1,
            data: bytes.to_vec(),
        })
    }
}

impl FromEncapsulation for RectProps {
    type Output = Self;

    fn from_encapsulation(body: Encapsulation) -> Result<Self::Output, Error> {
        let mut read_bytes = 0;
        Self::from_bytes(&body.data, &mut read_bytes)
    }
}


pub trait Demo : IceObject {
    fn say_hello(&mut self) -> Result<(), Error>;
    fn say(&mut self, text: &String) -> Result<(), Error>;
    fn calc_rect(&mut self, rc: &Rect) -> Result<RectProps, Error>;
}

pub struct DemoPrx {
    proxy: Proxy
}

impl IceObject for DemoPrx {
    const TYPE_ID: &'static str = "::RustDemo::Demo";
    const NAME: &'static str = "demo";

    fn dispatch(&mut self, op: &str, mode: u8, params: &Encapsulation) -> Result<ReplyData, Error> {
        let req = self.proxy.create_request(&DemoPrx::NAME, op, mode, params);
        self.proxy.make_request(&req)
    }

}

impl Demo for DemoPrx {
    fn say_hello(&mut self) -> Result<(), Error> { 
        self.dispatch(&String::from("sayHello"), 0, &Encapsulation::empty())?;
        Ok(())
    }
    fn say(&mut self, text: &String) -> Result<(), Error> { 
        self.dispatch(&String::from("say"), 0, &text.as_encapsulation()?)?;
        Ok(())
    }
    fn calc_rect(&mut self, rc: &Rect) -> Result<RectProps, Error> { 
        let reply = self.dispatch(&String::from("calcRect"), 0, &rc.as_encapsulation()?)?;
        RectProps::from_encapsulation(reply.body)
    }
}

impl DemoPrx {
    pub fn checked_cast(proxy: Proxy) -> Result<DemoPrx, Error> {
        let mut demo_proxy = DemoPrx {
            proxy: proxy
        };

        if !demo_proxy.ice_is_a()? {
            return Err(Error::TcpError);
        }

        Ok(demo_proxy)
    }
}

