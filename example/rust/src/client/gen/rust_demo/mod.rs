// This file has been generated.
use ice_rs::encoding::IceSize;
use ice_rs::encoding::{ToBytes, FromBytes};
use ice_rs::errors::*;
use ice_rs::iceobject::IceObject;
use ice_rs::protocol::{Encapsulation, ReplyData};
use ice_rs::proxy::Proxy;
use num_enum::TryFromPrimitive;
use std::collections::HashMap;
use std::convert::TryFrom;

type DoubleSeq = Vec<f64>;
type TestDict = HashMap<String, f64>;

#[derive(Debug, Copy, Clone, TryFromPrimitive, PartialEq)]
#[repr(i32)]
pub enum RectType {
    Rect = 0,
    Square = 1,
}

impl ToBytes for RectType {
    fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut bytes = Vec::new();
        bytes.extend(IceSize{size: *self as i32}.to_bytes()?);
        Ok(bytes)
    }
}
impl FromBytes for RectType {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Box<dyn std::error::Error>> {
        let mut read = 0;
        let enum_value =  IceSize::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?.size;
        *read_bytes = *read_bytes + read;

        match RectType::try_from(enum_value) {
            Ok(enum_type) => Ok(enum_type),
            _ => Err(Box::new(ProtocolError {}))
        }
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
    fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut bytes = Vec::new();
        bytes.extend(self.left.to_bytes()?);
        bytes.extend(self.right.to_bytes()?);
        bytes.extend(self.top.to_bytes()?);
        bytes.extend(self.bottom.to_bytes()?);
        Ok(bytes)
    }
}
impl FromBytes for Rect {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Box<dyn std::error::Error>> {
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
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RectProps {
    pub width: i64,
    pub height: i64,
    pub rect_type: RectType ,
}

impl ToBytes for RectProps {
    fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut bytes = Vec::new();
        bytes.extend(self.width.to_bytes()?);
        bytes.extend(self.height.to_bytes()?);
        bytes.extend(self.rect_type.to_bytes()?);
        Ok(bytes)
    }
}
impl FromBytes for RectProps {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Box<dyn std::error::Error>> {
        let mut read = 0;
        let obj = Self{
            width:  i64::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?,
            height:  i64::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?,
            rect_type:  RectType ::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?,
        };
        *read_bytes = *read_bytes + read;
        Ok(obj)
    }
}

#[derive(Debug)]
pub struct DemoException {
    pub message: String,
}

impl std::fmt::Display for DemoException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DemoException")
    }
}

impl std::error::Error for DemoException {
}

impl ToBytes for DemoException {
    fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut bytes = Vec::new();
        bytes.extend(self.message.to_bytes()?);
        Ok(bytes)
    }
}
impl FromBytes for DemoException {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Box<dyn std::error::Error>> {
        let mut read = 0;
        let _flag = u8::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        let _type_name = String::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        let obj = Self{
            message:  String::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?,
        };
        *read_bytes = *read_bytes + read;
        Ok(obj)
    }
}
#[derive(Debug)]
pub struct DerivedDemoException {
    pub detail: String,
    pub fatal: bool,
    pub extends: DemoException,
}

impl std::fmt::Display for DerivedDemoException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DerivedDemoException")
    }
}

impl std::error::Error for DerivedDemoException {
}

impl ToBytes for DerivedDemoException {
    fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut bytes = Vec::new();
        bytes.extend(self.detail.to_bytes()?);
        bytes.extend(self.fatal.to_bytes()?);
        Ok(bytes)
    }
}
impl FromBytes for DerivedDemoException {
    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Box<dyn std::error::Error>> {
        let mut read = 0;
        let _flag = u8::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        let _type_name = String::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
        let obj = Self{
            detail:  String::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?,
            fatal:  bool::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?,
            extends:  DemoException::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?,
        };
        *read_bytes = *read_bytes + read;
        Ok(obj)
    }
}

pub trait AnotherDemo : IceObject{
    fn base_exception(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}
pub struct AnotherDemoPrx {
    pub proxy: Proxy,
    pub id: String,
}

impl IceObject for AnotherDemoPrx {
    const TYPE_ID: &'static str = "::RustDemo::AnotherDemo";
    fn dispatch<T: 'static + std::fmt::Debug + std::fmt::Display + FromBytes>(&mut self, op: &str, mode: u8, params: &Encapsulation) -> Result<ReplyData, Box<dyn std::error::Error>> {
        let req = self.proxy.create_request(&self.id, op, mode, params);
        self.proxy.make_request::<T>(&req)
    }
}

impl AnotherDemo for AnotherDemoPrx {
    fn base_exception(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let  bytes = Vec::new();
        self.dispatch::<DemoException>(&String::from("baseException"), 0, &Encapsulation::from(bytes))?;

        Ok(())
    }

}

impl  AnotherDemoPrx {
    pub fn checked_cast(id: &str, proxy: Proxy) -> Result<Self, Box<dyn std::error::Error>> {
        let mut my_proxy = Self {
            proxy: proxy,
            id: String::from(id)
        };

        if !my_proxy.ice_is_a()? {
            return Err(Box::new(ProtocolError {}));
        }
        Ok(my_proxy)
    }
}

pub trait Demo : IceObject{
    fn say_hello(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn say(&mut self, text: &String) -> Result<(), Box<dyn std::error::Error>>;
    fn calc_rect(&mut self, rc: &Rect ) -> Result<RectProps, Box<dyn std::error::Error>>;
    fn add(&mut self, x: f64, y: f64) -> Result<f64, Box<dyn std::error::Error>>;
    fn square(&mut self, x: f64, y: &mut f64) -> Result<(), Box<dyn std::error::Error>>;
    fn square_root(&mut self, x: f64, y: &mut f64) -> Result<bool, Box<dyn std::error::Error>>;
    fn sum(&mut self, x: &DoubleSeq ) -> Result<f64, Box<dyn std::error::Error>>;
    fn get_hello(&mut self, x: &TestDict ) -> Result<f64, Box<dyn std::error::Error>>;
    fn native_exception(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn base_exception(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn derived_exception(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn optional_square(&mut self, n: Option<f64>) -> Result<Option<f64>, Box<dyn std::error::Error>>;
}
pub struct DemoPrx {
    pub proxy: Proxy,
    pub id: String,
}

impl IceObject for DemoPrx {
    const TYPE_ID: &'static str = "::RustDemo::Demo";
    fn dispatch<T: 'static + std::fmt::Debug + std::fmt::Display + FromBytes>(&mut self, op: &str, mode: u8, params: &Encapsulation) -> Result<ReplyData, Box<dyn std::error::Error>> {
        let req = self.proxy.create_request(&self.id, op, mode, params);
        self.proxy.make_request::<T>(&req)
    }
}

impl Demo for DemoPrx {
    fn say_hello(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let  bytes = Vec::new();
        self.dispatch::<ProtocolError>(&String::from("sayHello"), 0, &Encapsulation::from(bytes))?;

        Ok(())
    }

    fn say(&mut self, text: &String) -> Result<(), Box<dyn std::error::Error>> {
        let mut bytes = Vec::new();
        bytes.extend(text.to_bytes()?);
        self.dispatch::<ProtocolError>(&String::from("say"), 0, &Encapsulation::from(bytes))?;

        Ok(())
    }

    fn calc_rect(&mut self, rc: &Rect ) -> Result<RectProps, Box<dyn std::error::Error>> {
        let mut bytes = Vec::new();
        bytes.extend(rc.to_bytes()?);
        let reply = self.dispatch::<ProtocolError>(&String::from("calcRect"), 0, &Encapsulation::from(bytes))?;

        let mut read_bytes: i32 = 0;
        RectProps::from_bytes(&reply.body.data[read_bytes as usize..reply.body.data.len()], &mut read_bytes)
    }

    fn add(&mut self, x: f64, y: f64) -> Result<f64, Box<dyn std::error::Error>> {
        let mut bytes = Vec::new();
        bytes.extend(x.to_bytes()?);
        bytes.extend(y.to_bytes()?);
        let reply = self.dispatch::<ProtocolError>(&String::from("add"), 0, &Encapsulation::from(bytes))?;

        let mut read_bytes: i32 = 0;
        f64::from_bytes(&reply.body.data[read_bytes as usize..reply.body.data.len()], &mut read_bytes)
    }

    fn square(&mut self, x: f64, y: &mut f64) -> Result<(), Box<dyn std::error::Error>> {
        let mut bytes = Vec::new();
        bytes.extend(x.to_bytes()?);
        let reply = self.dispatch::<ProtocolError>(&String::from("square"), 0, &Encapsulation::from(bytes))?;

        let mut read_bytes: i32 = 0;
        *y = f64::from_bytes(&reply.body.data[read_bytes as usize..reply.body.data.len()], &mut read_bytes)?;
        Ok(())
    }

    fn square_root(&mut self, x: f64, y: &mut f64) -> Result<bool, Box<dyn std::error::Error>> {
        let mut bytes = Vec::new();
        bytes.extend(x.to_bytes()?);
        let reply = self.dispatch::<ProtocolError>(&String::from("squareRoot"), 0, &Encapsulation::from(bytes))?;

        let mut read_bytes: i32 = 0;
        *y = f64::from_bytes(&reply.body.data[read_bytes as usize..reply.body.data.len()], &mut read_bytes)?;
        bool::from_bytes(&reply.body.data[read_bytes as usize..reply.body.data.len()], &mut read_bytes)
    }

    fn sum(&mut self, x: &DoubleSeq ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut bytes = Vec::new();
        bytes.extend(x.to_bytes()?);
        let reply = self.dispatch::<ProtocolError>(&String::from("sum"), 0, &Encapsulation::from(bytes))?;

        let mut read_bytes: i32 = 0;
        f64::from_bytes(&reply.body.data[read_bytes as usize..reply.body.data.len()], &mut read_bytes)
    }

    fn get_hello(&mut self, x: &TestDict ) -> Result<f64, Box<dyn std::error::Error>> {
        let mut bytes = Vec::new();
        bytes.extend(x.to_bytes()?);
        let reply = self.dispatch::<ProtocolError>(&String::from("getHello"), 0, &Encapsulation::from(bytes))?;

        let mut read_bytes: i32 = 0;
        f64::from_bytes(&reply.body.data[read_bytes as usize..reply.body.data.len()], &mut read_bytes)
    }

    fn native_exception(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let  bytes = Vec::new();
        self.dispatch::<ProtocolError>(&String::from("nativeException"), 0, &Encapsulation::from(bytes))?;

        Ok(())
    }

    fn base_exception(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let  bytes = Vec::new();
        self.dispatch::<DemoException>(&String::from("baseException"), 0, &Encapsulation::from(bytes))?;

        Ok(())
    }

    fn derived_exception(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let  bytes = Vec::new();
        self.dispatch::<DerivedDemoException>(&String::from("derivedException"), 0, &Encapsulation::from(bytes))?;

        Ok(())
    }

    fn optional_square(&mut self, n: Option<f64>) -> Result<Option<f64>, Box<dyn std::error::Error>> {
        let mut bytes = Vec::new();
        bytes.extend(n.to_bytes()?);
        let reply = self.dispatch::<ProtocolError>(&String::from("optionalSquare"), 0, &Encapsulation::from(bytes))?;

        let mut read_bytes: i32 = 0;
        Option::<f64>::from_bytes(&reply.body.data[read_bytes as usize..reply.body.data.len()], &mut read_bytes)
    }

}

impl  DemoPrx {
    pub fn checked_cast(id: &str, proxy: Proxy) -> Result<Self, Box<dyn std::error::Error>> {
        let mut my_proxy = Self {
            proxy: proxy,
            id: String::from(id)
        };

        if !my_proxy.ice_is_a()? {
            return Err(Box::new(ProtocolError {}));
        }
        Ok(my_proxy)
    }
}

