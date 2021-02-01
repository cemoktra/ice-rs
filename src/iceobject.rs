use crate::{errors::ProtocolError, protocol::{Encapsulation, ReplyData}};
use crate::encoding::{FromBytes, ToBytes};
use std::collections::HashMap;
use async_trait::async_trait;

/// The `IceObject` trait is a base trait for all
/// ice interfaces. It implements functions that
/// are equal to all ice interfaces.
#[async_trait]
pub trait IceObject {
    const TYPE_ID: &'static str;

    async fn ice_ping(&mut self) -> Result<(), Box<dyn std::error::Error + Sync + Send>>
    {
        self.dispatch::<ProtocolError>(&String::from("ice_ping"), 1, &Encapsulation::empty(), None).await?;
        Ok(())
    }

    async fn ice_is_a(&mut self) -> Result<bool, Box<dyn std::error::Error + Sync + Send>> {
        let reply = self.dispatch::<ProtocolError>(&String::from("ice_isA"), 1, &Encapsulation::from(Self::TYPE_ID.to_bytes()?), None).await?;
        let mut read_bytes: i32 = 0;
        bool::from_bytes(&reply.body.data, &mut read_bytes)
    }

    async fn ice_id(&mut self) -> Result<String, Box<dyn std::error::Error + Sync + Send>>
    {
        let reply = self.dispatch::<ProtocolError>(&String::from("ice_id"), 1, &Encapsulation::empty(), None).await?;
        let mut read_bytes: i32 = 0;
        String::from_bytes(&reply.body.data, &mut read_bytes)
    }

    async fn ice_ids(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error + Sync + Send>>
    {
        let reply = self.dispatch::<ProtocolError>(&String::from("ice_ids"), 1, &Encapsulation::empty(), None).await?;
        let mut read_bytes: i32 = 0;
        Vec::from_bytes(&reply.body.data, &mut read_bytes)
    }

    async fn dispatch<T: 'static + std::fmt::Debug + std::fmt::Display + FromBytes + Sync + Send>(&mut self, op: &str, mode: u8, params: &Encapsulation, context: Option<HashMap<String, String>>) -> Result<ReplyData, Box<dyn std::error::Error + Sync + Send>>;
}