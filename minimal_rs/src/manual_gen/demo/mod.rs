use ice_rs::errors::Error;
use ice_rs::encoding::{
    AsBytes, FromBytes, AsEncapsulation, FromEncapsulation,
    encode_long, decode_long, decode_size
};
use ice_rs::protocol::{Encapsulation, ReplyData};
use ice_rs::proxy::Proxy;


// only for enums
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;


#[derive(Debug, Copy, Clone, TryFromPrimitive)]
#[repr(i32)]
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
    fn from_bytes(bytes: &[u8]) -> Result<(Self, i32), Error> {
        let width = decode_long(&bytes[0..8])?;
        let height = decode_long(&bytes[8..16])?;
        let (enum_value, _) = decode_size(&bytes[16..bytes.len()]);
        let enum_type = match RectType::try_from(enum_value) {
            Ok(enum_type) => enum_type,
            _ => {
                return Err(Error::CannotDeserialize)
            }
        };

        Ok((RectProps{
            width: width,
            height: height,
            rect_type: enum_type
        }, 17))
    }
}

impl AsEncapsulation for Rect {
    fn as_encapsulation(&self) -> Result<Encapsulation, Error> {
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
        let (props, _) = RectProps::from_bytes(&body.data)?;
        Ok(props)
    }
}

pub struct HelloPrx
{
    proxy: Proxy
}


impl Hello for HelloPrx {
    fn ice_ping(&mut self) -> Result<(), Error>
    {
        self.dispatch(&String::from("ice_ping"), 1, Encapsulation::empty())?;
        Ok(())
    }

    fn ice_is_a(&mut self) -> Result<bool, Error> {
        let reply = self.dispatch(&String::from("ice_isA"), 1, HelloPrx::TYPE_ID.as_encapsulation()?)?;
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

impl HelloPrx {
    const TYPE_ID: &'static str = "::Demo::Hello";
    const NAME: &'static str = "hello";

    pub fn checked_cast(proxy: Proxy) -> Result<HelloPrx, Error> {
        let mut hello_proxy = HelloPrx {
            proxy: proxy
        };

        if !hello_proxy.ice_is_a()? {
            return Err(Error::TcpCannotConnect);
        }

        Ok(hello_proxy)
    }

    fn dispatch(&mut self, op: &str, mode: u8, params: Encapsulation) -> Result<ReplyData, Error> {
        let req = self.proxy.create_request(
            &HelloPrx::NAME, 
            op,
            mode, 
            params
        );
        self.proxy.make_request(&req)
    }
}