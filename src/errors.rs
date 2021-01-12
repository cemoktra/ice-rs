use std::fmt::Display;
use crate::encoding::FromBytes;

/// A `ProtocolError` indicates a problem related to the
/// ice protocol. It may be unexpected messages or problems
/// while encoding or decoding objects.
#[derive(Debug)]
pub struct ProtocolError {}

/// A `ParsingError` appears when a problem occurs parsing ice
/// files.
#[derive(Debug)]
pub struct ParsingError {}

/// A `PropertyError` appears when a requested property is not
/// existing.
#[derive(Debug)]
pub struct PropertyError {
    missing_key: String
}

impl PropertyError {
    pub fn new(missing_key: &str) -> PropertyError {
        PropertyError {
            missing_key: String::from(missing_key)
        }
    }
}

/// A `RemoteException` is raised when the remote application
/// raises any error that is not an `UserError`.
#[derive(Debug)]
pub struct RemoteException {
    pub cause: String
}

/// A `UserError` is an error that is defined in ice files.
/// The generic type will be the defined error struct.
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

impl std::fmt::Display for PropertyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PropertyError!")
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
impl std::error::Error for PropertyError {}
impl<T: std::fmt::Debug + Display + FromBytes> std::error::Error for UserError<T> {}

// dummy needed, but should not get called
// consider making it panic
impl FromBytes for ProtocolError {
    fn from_bytes(_bytes: &[u8], _read_bytes: &mut i32) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        Ok(ProtocolError {})
    }
}