use crate::protocol::{RequestData, MessageType};
use crate::errors::Error;


pub trait Transport {
    fn read_message(&mut self) -> Result<MessageType, Error>;
    fn validate_connection(&mut self) -> Result<(), Error>;
    fn make_request(&mut self, request: &RequestData) -> Result<(), Error>;
}