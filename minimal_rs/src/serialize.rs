use crate::errors::Error;

pub trait Serialize {
    fn to_bytes(&self) -> Result<Vec<u8>, Error>;
}