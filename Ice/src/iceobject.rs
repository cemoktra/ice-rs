use crate::errors::Error;
use crate::protocol::{Encapsulation, ReplyData};
use crate::encoding::{FromBytes, ToBytes};

pub trait IceObject {
    const TYPE_ID: &'static str;
    const NAME: &'static str;

    fn ice_ping(&mut self) -> Result<(), Error>
    {
        self.dispatch(&String::from("ice_ping"), 1, &Encapsulation::empty())?;
        Ok(())
    }

    fn ice_is_a(&mut self) -> Result<bool, Error> {
        let reply = self.dispatch(&String::from("ice_isA"), 1, &Encapsulation::from(Self::TYPE_ID.to_bytes()?))?;
        let mut read_bytes: i32 = 0;
        bool::from_bytes(&reply.body.data, &mut read_bytes)
    }

    fn ice_id(&mut self) -> Result<String, Error>
    {
        let reply = self.dispatch(&String::from("ice_id"), 1, &Encapsulation::empty())?;
        let mut read_bytes: i32 = 0;
        String::from_bytes(&reply.body.data, &mut read_bytes)
    }

    fn ice_ids(&mut self) -> Result<Vec<String>, Error>
    {
        let reply = self.dispatch(&String::from("ice_ids"), 1, &Encapsulation::empty())?;
        let mut read_bytes: i32 = 0;
        Vec::from_bytes(&reply.body.data, &mut read_bytes)
    }

    fn dispatch(&mut self, op: &str, mode: u8, params: &Encapsulation) -> Result<ReplyData, Error>;
}