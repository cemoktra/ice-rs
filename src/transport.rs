use crate::{errors::ProtocolError, protocol::{Header, MessageType, ReplyData, RequestData}};
use crate::encoding::{ToBytes, FromBytes};

pub trait Transport {
    fn read(&mut self) -> std::io::Result<&[u8]>;
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>;
    
    fn read_message(&mut self) -> Result<MessageType, Box<dyn std::error::Error>>
    {
        let buffer = self.read()?;
        let bytes = buffer.len();
        let mut read: i32 = 0;
        let header = Header::from_bytes(&buffer[read as usize..bytes], &mut read)?;

        match header.message_type {
            2 => {
                let reply = ReplyData::from_bytes(&buffer[read as usize..bytes as usize], &mut read)?;
                Ok(MessageType::Reply(header, reply))
            }
            3 => Ok(MessageType::ValidateConnection(header)),
            _ => Err(Box::new(ProtocolError::new(&format!("TCP: Unsuppored reply message type: {}", header.message_type))))
        }
    }

    fn validate_connection(&mut self) -> Result<(), Box<dyn std::error::Error>>
    {
        let header = Header::new(0, 14);
        let mut bytes = header.to_bytes()?;
        let written = self.write(&mut bytes)?;
        if written != header.message_size as usize {
            return Err(Box::new(ProtocolError::new("TCP: Could not validate connection")))
        }

        Ok(())
    }

    fn make_request(&mut self, request: &RequestData) -> Result<(), Box<dyn std::error::Error>>
    {
        let req_bytes = request.to_bytes()?;
        let header = Header::new(0, 14 + req_bytes.len() as i32);
        let mut bytes = header.to_bytes()?;
        bytes.extend(req_bytes);

        let written = self.write(&mut bytes)?;
        if written != header.message_size as usize {
            return Err(Box::new(ProtocolError::new(&format!("TCP: Error writing request {}", request.request_id))))
        }
        Ok(())
    }
}