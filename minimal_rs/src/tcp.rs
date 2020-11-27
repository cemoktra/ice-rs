use std::io::prelude::*;
use std::net::TcpStream;

use crate::errors::Error;
use crate::deserialize::Deserialize;
use crate::serialize::Serialize;
use crate::protocol::{Header, MessageType, RequestData, ReplyData};
use crate::transport::Transport;

pub struct TcpTransport {
    stream: TcpStream,
    buffer: Vec<u8>
}

impl std::convert::From<std::io::Error> for Error {
    fn from(_err: std::io::Error) -> Error {
        Error::TcpCannotConnect
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
            _ => Err(Error::TcpCannotConnect)
        }
    }
}

impl Transport for TcpTransport {
    fn read_message(&mut self) -> Result<MessageType, Error>
    {
        let bytes = self.stream.read(&mut self.buffer)?;
        let header = Header::from_bytes(&self.buffer[0..bytes])?;

        match header.message_type {
            2 => {
                Ok(MessageType::Reply(header, ReplyData::from_bytes(&self.buffer[14..bytes])?))
            }
            3 => Ok(MessageType::ValidateConnection(header)),
            _ => Err(Error::UnknownMessageType)
        }
    }

    fn validate_connection(&mut self) -> Result<(), Error>
    {
        let header = Header {
            magic: String::from("IceP"),
            protocol_major: 1,
            protocol_minor: 0,
            encoding_major: 1,
            encoding_minor: 0,
            message_type: 3,
            compression_status: 0,
            message_size: 14
        };
        let bytes = header.to_bytes()?;
        let written = self.stream.write(&bytes)?;
        if written != header.message_size as usize {
            return Err(Error::MessageWriteError);
        }

        Ok(())
    }

    fn make_request(&mut self, request: &RequestData) -> Result<(), Error>
    {
        let req_bytes = request.to_bytes()?;
        let header = Header {
            magic: String::from("IceP"),
            protocol_major: 1,
            protocol_minor: 0,
            encoding_major: 1,
            encoding_minor: 0,
            message_type: 0,
            compression_status: 0,
            message_size: 14 + req_bytes.len() as i32
        };
        let mut bytes = header.to_bytes()?;
        bytes.extend(req_bytes);

        let written = self.stream.write(&bytes)?;
        if written != header.message_size as usize {
            return Err(Error::MessageWriteError);
        }
        Ok(())
    }
}