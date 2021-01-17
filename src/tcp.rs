use std::io::prelude::*;
use std::net::TcpStream;

use crate::protocol::MessageType;
use crate::transport::Transport;
use crate::errors::*;

pub struct TcpTransport {
    stream: TcpStream,
    buffer: Vec<u8>
}

impl TcpTransport {
    pub fn new(address: &str) -> Result<TcpTransport, Box<dyn std::error::Error>>
    {
        let mut transport = TcpTransport {
            stream: TcpStream::connect(address)?,
            buffer: vec![0; 4096]
        };

        match transport.read_message()? {
            MessageType::ValidateConnection(_) => Ok(transport),
            _ => Err(Box::new(ProtocolError::new("TCP: Failed to validate new connection")))
        }
    }
}

impl Transport for TcpTransport {
    fn read(&mut self) -> std::io::Result<&[u8]> {
        let bytes = self.stream.read(&mut self.buffer)?;
        Ok(&self.buffer[0..bytes])
    }

    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>
    {
        self.stream.write(&buf)
    }
}