use crate::errors::Error;

pub trait Deserialize {
    fn from_bytes(bytes: &[u8]) -> Result<Self, Error>  where Self: Sized;
}