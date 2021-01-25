use crate::{errors::ProtocolError, protocol::{Encapsulation, ReplyData}};
use crate::encoding::{FromBytes, ToBytes};
use std::collections::HashMap;


/// The `IceObject` trait is a base trait for all
/// ice interfaces. It implements functions that
/// are equal to all ice interfaces.
pub trait IceObject {
    const TYPE_ID: &'static str;

    fn ice_ping(&mut self) -> Result<(), Box<dyn std::error::Error>>
    {
        self.dispatch::<ProtocolError>(&String::from("ice_ping"), 1, &Encapsulation::empty(), None)?;
        Ok(())
    }

    fn ice_is_a(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let reply = self.dispatch::<ProtocolError>(&String::from("ice_isA"), 1, &Encapsulation::from(Self::TYPE_ID.to_bytes()?), None)?;
        let mut read_bytes: i32 = 0;
        bool::from_bytes(&reply.body.data, &mut read_bytes)
    }

    fn ice_id(&mut self) -> Result<String, Box<dyn std::error::Error>>
    {
        let reply = self.dispatch::<ProtocolError>(&String::from("ice_id"), 1, &Encapsulation::empty(), None)?;
        let mut read_bytes: i32 = 0;
        String::from_bytes(&reply.body.data, &mut read_bytes)
    }

    fn ice_ids(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>>
    {
        let reply = self.dispatch::<ProtocolError>(&String::from("ice_ids"), 1, &Encapsulation::empty(), None)?;
        let mut read_bytes: i32 = 0;
        Vec::from_bytes(&reply.body.data, &mut read_bytes)
    }

    fn dispatch<T: 'static + std::fmt::Debug + std::fmt::Display + FromBytes>(&mut self, op: &str, mode: u8, params: &Encapsulation, context: Option<HashMap<String, String>>) -> Result<ReplyData, Box<dyn std::error::Error>>;
}