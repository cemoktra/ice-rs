use crate::{errors::ProtocolError, protocol::{Encapsulation, ReplyData}};
use crate::encoding::{FromBytes, ToBytes};

pub trait IceObject {
    const TYPE_ID: &'static str;
    const NAME: &'static str;

    fn ice_ping(&mut self) -> Result<(), Box<dyn std::error::Error>>
    {
        self.dispatch::<ProtocolError>(&String::from("ice_ping"), 1, &Encapsulation::empty())?;
        Ok(())
    }

    fn ice_is_a(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let reply = self.dispatch::<ProtocolError>(&String::from("ice_isA"), 1, &Encapsulation::from(Self::TYPE_ID.to_bytes()?))?;
        let mut read_bytes: i32 = 0;
        bool::from_bytes(&reply.body.data, &mut read_bytes)
    }

    fn ice_id(&mut self) -> Result<String, Box<dyn std::error::Error>>
    {
        let reply = self.dispatch::<ProtocolError>(&String::from("ice_id"), 1, &Encapsulation::empty())?;
        let mut read_bytes: i32 = 0;
        String::from_bytes(&reply.body.data, &mut read_bytes)
    }

    fn ice_ids(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error>>
    {
        let reply = self.dispatch::<ProtocolError>(&String::from("ice_ids"), 1, &Encapsulation::empty())?;
        let mut read_bytes: i32 = 0;
        Vec::from_bytes(&reply.body.data, &mut read_bytes)
    }

    fn dispatch<T: 'static + std::fmt::Debug + std::fmt::Display + FromBytes>(&mut self, op: &str, mode: u8, params: &Encapsulation) -> Result<ReplyData, Box<dyn std::error::Error>>;
}