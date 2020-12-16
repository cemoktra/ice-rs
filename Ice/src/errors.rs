use crate::protocol::Encapsulation;


#[derive(Debug)]
pub struct ProtocolError {}

#[derive(Debug)]
pub struct ParsingError {}

#[derive(Debug)]
pub struct RemoteException {
    pub cause: String
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

impl std::error::Error for ProtocolError {}
impl std::error::Error for ParsingError {}
impl std::error::Error for RemoteException {}

pub trait UserError<T: std::error::Error> {
    fn parse_user_error(&self, data: &Encapsulation) -> Result<T, Box<dyn std::error::Error>>;
}