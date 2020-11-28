use ice_rs::errors::Error;

pub trait Hello {
    // base ice
    fn ice_is_a(&mut self) -> Result<bool, Error>;
    // hello interface
    fn say_hello(&mut self) -> Result<(), Error>;
}