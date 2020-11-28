use ice_rs::errors::Error;

pub trait Hello {
    // base ice
    fn ice_ping(&mut self) -> Result<(), Error>;
    fn ice_is_a(&mut self) -> Result<bool, Error>;
    fn ice_id(&mut self) -> Result<String, Error>;
    fn ice_ids(&mut self) -> Result<Vec<String>, Error>;
    // hello interface
    fn say_hello(&mut self) -> Result<(), Error>;
}