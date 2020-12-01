use ice_rs::errors::Error;
use ice_rs::encoding::{
    ToBytes, FromBytes, AsEncapsulation, FromEncapsulation, IceSize
};
use ice_rs::protocol::{Encapsulation, ReplyData};
use ice_rs::proxy::Proxy;


// only for enums
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;


#[derive(Debug, Copy, Clone, TryFromPrimitive, PartialEq)]
#[repr(i32)]
pub enum RectType {
    Rect,
    Square
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rect {
    pub left: i64,
    pub right: i64,
    pub top: i64,
    pub bottom: i64
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RectProps {
    pub width: i64,
    pub height: i64,
    pub rect_type: RectType
}


pub trait Demo {
    // base ice
    fn ice_ping(&mut self) -> Result<(), Error>;
    fn ice_is_a(&mut self) -> Result<bool, Error>;
    fn ice_id(&mut self) -> Result<String, Error>;
    fn ice_ids(&mut self) -> Result<Vec<String>, Error>;
    // demo interface
    fn say_hello(&mut self) -> Result<(), Error>;
    fn say(&mut self, text: &str) -> Result<(), Error>;
    fn calc_rect(&mut self, rc: Rect) -> Result<RectProps, Error>;
}

impl ToBytes for Rect {
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes = self.left.to_bytes()?;
        bytes.extend(self.right.to_bytes()?);
        bytes.extend(self.top.to_bytes()?);
        bytes.extend(self.bottom.to_bytes()?);
        Ok(bytes)
    }
}

impl FromBytes for Rect {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error> {
        let mut read = 0;
        let left =  i64::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        let right =  i64::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        let top =  i64::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        let bottom =  i64::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        *read_bytes = *read_bytes + read;

        Ok(Rect {
            left: left,
            right: right,
            top: top,
            bottom:  bottom,
        })
    }
}

impl ToBytes for RectProps {
    fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        let mut bytes = self.width.to_bytes()?;
        bytes.extend(self.height.to_bytes()?);
        bytes.extend(IceSize{size: self.rect_type as i32}.to_bytes()?);
        Ok(bytes)
    }
}

impl FromBytes for RectProps {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error> {
        let mut read = 0;
        let width =  i64::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        let height =  i64::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        let enum_value =  IceSize::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?.size;
        let enum_type = match RectType::try_from(enum_value) {
            Ok(enum_type) => enum_type,
            _ => {
                return Err(Error::DecodingError)
            }
        };
        *read_bytes = *read_bytes + read;

        Ok(RectProps{
            width: width,
            height: height,
            rect_type: enum_type
        })
    }
}

impl AsEncapsulation for Rect {
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

impl FromEncapsulation for Rect {
    type Output = Rect;

    fn from_encapsulation(body: Encapsulation) -> Result<Self::Output, Error> {
        let mut read_bytes = 0;
        Rect::from_bytes(&body.data, &mut read_bytes)
    }
}

impl AsEncapsulation for RectProps {
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

impl FromEncapsulation for RectProps {
    type Output = RectProps;

    fn from_encapsulation(body: Encapsulation) -> Result<Self::Output, Error> {
        let mut read_bytes = 0;
        RectProps::from_bytes(&body.data, &mut read_bytes)
    }
}

pub struct DemoPrx
{
    proxy: Proxy
}


impl Demo for DemoPrx {
    fn ice_ping(&mut self) -> Result<(), Error>
    {
        self.dispatch(&String::from("ice_ping"), 1, Encapsulation::empty())?;
        Ok(())
    }

    fn ice_is_a(&mut self) -> Result<bool, Error> {
        let reply = self.dispatch(&String::from("ice_isA"), 1, DemoPrx::TYPE_ID.as_encapsulation()?)?;
        bool::from_encapsulation(reply.body)
    }

    fn ice_id(&mut self) -> Result<String, Error>
    {
        let reply = self.dispatch(&String::from("ice_id"), 1, Encapsulation::empty())?;
        String::from_encapsulation(reply.body)
    }

    fn ice_ids(&mut self) -> Result<Vec<String>, Error>
    {
        let reply = self.dispatch(&String::from("ice_ids"), 1, Encapsulation::empty())?;
        Vec::from_encapsulation(reply.body)
    }

    fn say_hello(&mut self) -> Result<(), Error> {
        self.dispatch(&String::from("sayHello"), 0, Encapsulation::empty())?;
        Ok(())
    }

    fn say(&mut self, text: &str) -> Result<(), Error> {
        self.dispatch(&String::from("say"), 0, text.as_encapsulation()?)?;
        Ok(())
    }

    fn calc_rect(&mut self, rc: Rect) -> Result<RectProps, Error> {
        let reply = self.dispatch(&String::from("calcRect"), 0, rc.as_encapsulation()?)?;
        RectProps::from_encapsulation(reply.body)
    }
}

impl DemoPrx {
    const TYPE_ID: &'static str = "::RustDemo::Demo";
    const NAME: &'static str = "demo";

    pub fn checked_cast(proxy: Proxy) -> Result<DemoPrx, Error> {
        let mut demo_proxy = DemoPrx {
            proxy: proxy
        };

        if !demo_proxy.ice_is_a()? {
            return Err(Error::TcpError);
        }

        Ok(demo_proxy)
    }

    fn dispatch(&mut self, op: &str, mode: u8, params: Encapsulation) -> Result<ReplyData, Error> {
        let req = self.proxy.create_request(
            &DemoPrx::NAME, 
            op,
            mode, 
            params
        );
        self.proxy.make_request(&req)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rect_encoding() {
        let mut read_bytes = 0;
        let rect = Rect {
            left: 0,
            right: 100,
            top: 0,
            bottom: 50
        };
        let bytes = rect.to_bytes().expect("Cannot encode test rect");
        let decoded = Rect::from_bytes(&bytes, &mut read_bytes).expect("Cannot decode test rect");

        assert_eq!(32, read_bytes);
        assert_eq!(rect.left, decoded.left);
        assert_eq!(rect.right, decoded.right);
        assert_eq!(rect.top, decoded.top);
        assert_eq!(rect.bottom, decoded.bottom);
    }

    #[test]
    fn test_rect_props_encoding() {
        let mut read_bytes = 0;
        let props = RectProps {
            width: 0,
            height: 100,
            rect_type: RectType::Rect
        };
        let bytes = props.to_bytes().expect("Cannot encode test rect props");
        let decoded = RectProps::from_bytes(&bytes, &mut read_bytes).expect("Cannot decode test rect props");

        assert_eq!(17, read_bytes);
        assert_eq!(props.width, decoded.width);
        assert_eq!(props.height, decoded.height);
        assert_eq!(props.rect_type, decoded.rect_type);
    }

    #[test]
    fn test_rect_encapsulation() {
        let rect = Rect {
            left: 0,
            right: 100,
            top: 0,
            bottom: 50
        };
        let encapsulation = rect.as_encapsulation().expect("Cannot encapsulate test rect");
        let decoded = Rect::from_encapsulation(encapsulation).expect("Cannot decode test rect from encapsulation");
        assert_eq!(rect, decoded);
    }

    #[test]
    fn test_rect_props_encapsulation() {
        let props = RectProps {
            width: 0,
            height: 100,
            rect_type: RectType::Rect
        };
        let encapsulation = props.as_encapsulation().expect("Cannot encapsulate test rect props");
        let decoded = RectProps::from_encapsulation(encapsulation).expect("Cannot decode test rect props from encapsulation");
        assert_eq!(props, decoded);
    }
}