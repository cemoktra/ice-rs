use crate::protocol::{RequestData, MessageType};


pub trait Transport {
    fn read_message(&mut self) -> Result<MessageType, Box<dyn std::error::Error>>;
    fn validate_connection(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn make_request(&mut self, request: &RequestData) -> Result<(), Box<dyn std::error::Error>>;
}