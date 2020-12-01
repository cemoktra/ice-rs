use std::io::prelude::*;
use std::net::TcpStream;

use crate::errors::Error;
use crate::encoding::{AsBytes, FromBytes};
use crate::protocol::{Header, MessageType, RequestData, ReplyData};
use crate::transport::Transport;

pub struct TcpTransport {
    stream: TcpStream,
    buffer: Vec<u8>
}

impl std::convert::From<std::io::Error> for Error {
    fn from(_err: std::io::Error) -> Error {
        Error::TcpError
    }
}

impl TcpTransport {
    pub fn new(address: &str) -> Result<TcpTransport, Error>
    {
        let mut transport = TcpTransport {
            stream: TcpStream::connect(address)?,
            buffer: vec![0; 4096]
        };

        match transport.read_message()? {
            MessageType::ValidateConnection(_) => Ok(transport),
            _ => Err(Error::TcpError)
        }
    }
}

impl Transport for TcpTransport {
    fn read_message(&mut self) -> Result<MessageType, Error>
    {
        let bytes = self.stream.read(&mut self.buffer)?;
        let mut read: i32 = 0;
        let header = Header::from_bytes(&self.buffer[read as usize..bytes], &mut read)?;

        match header.message_type {
            2 => {
                let reply = ReplyData::from_bytes(&self.buffer[read as usize..bytes as usize], &mut read)?;
                Ok(MessageType::Reply(header, reply))
            }
            3 => Ok(MessageType::ValidateConnection(header)),
            _ => Err(Error::ProtocolError)
        }
    }

    fn validate_connection(&mut self) -> Result<(), Error>
    {
        let header = Header::new(0, 14);
        let bytes = header.as_bytes()?;
        let written = self.stream.write(&bytes)?;
        if written != header.message_size as usize {
            return Err(Error::TcpError);
        }

        Ok(())
    }

    fn make_request(&mut self, request: &RequestData) -> Result<(), Error>
    {
        let req_bytes = request.as_bytes()?;
        let header = Header::new(0, 14 + req_bytes.len() as i32);
        let mut bytes = header.as_bytes()?;
        bytes.extend(req_bytes);

        let written = self.stream.write(&bytes)?;
        if written != header.message_size as usize {
            return Err(Error::TcpError);
        }
        Ok(())
    }
}