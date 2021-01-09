use crate::proxy::Proxy;
use crate::initdata::InitializationData;
use crate::errors::PropertyError;

/// The Communicator is a basic object in ZeroC Ice. Currently
/// this is more a stub that does dummy initialization.
pub struct Communicator {
    init_data: InitializationData
}

impl Communicator {
    pub fn new() -> Communicator {
        Communicator {
            init_data: InitializationData::new()
        }
    }

    pub fn string_to_proxy(&self, proxy_string: &str) -> Result<Proxy, Box<dyn std::error::Error>> {
        Proxy::new(proxy_string, &self.init_data.properties)
    }

    pub fn property_to_proxy(&self, property: &str) -> Result<Proxy, Box<dyn std::error::Error>> {
        match self.init_data.properties.get(property) {
            Some(value) => {
                Proxy::new(value, &self.init_data.properties)
            }
            None => {
                Err(Box::new(PropertyError {}))
            }
        }
    }
}

pub fn initialize(config_file: &str) -> Communicator {
    let mut init_data = InitializationData::new();
    init_data.properties.load(config_file).unwrap();
    Communicator{
        init_data: init_data,
    }
}