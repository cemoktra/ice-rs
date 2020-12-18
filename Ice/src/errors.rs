use std::fmt::Display;

use crate::encoding::FromBytes;

#[derive(Debug)]
pub struct ProtocolError {}

#[derive(Debug)]
pub struct ParsingError {}

#[derive(Debug)]
pub struct RemoteException {
    pub cause: String
}

#[derive(Debug)]
pub struct UserError<T: std::fmt::Display> {
    pub exception: T
}

impl std::fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ProtocolError!")
    }
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ParsingError!")
    }
}

impl std::fmt::Display for RemoteException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RemoteException: {}", self.cause)
    }
}

impl<T: Display> std::fmt::Display for UserError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.exception)
    }
}


impl std::error::Error for ProtocolError {}
impl std::error::Error for ParsingError {}
impl std::error::Error for RemoteException {}
impl<T: std::fmt::Debug + Display + FromBytes> std::error::Error for UserError<T> {}

// dummy needed, but should not get called
// consider making it panic
impl FromBytes for ProtocolError {
    fn from_bytes(_bytes: &[u8], _read_bytes: &mut i32) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        Ok(ProtocolError {})
    }
}