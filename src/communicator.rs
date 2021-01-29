use crate::{proxy::Proxy, proxy_factory::ProxyFactory};
use crate::initdata::InitializationData;
use crate::errors::PropertyError;

/// The Communicator is a basic object in ZeroC Ice. Currently
/// this is more a stub that does dummy initialization.
pub struct Communicator {
    pub init_data: InitializationData,
    proxy_factory: ProxyFactory
}

impl Communicator {
    pub fn new() -> Result<Communicator, Box<dyn std::error::Error>> {
        let init_data = InitializationData::new();
        let proxy_factory = ProxyFactory::new(&init_data.properties)?;
        Ok(Communicator {
            init_data,
            proxy_factory
        })
    }

    pub fn string_to_proxy(&mut self, proxy_string: &str) -> Result<Proxy, Box<dyn std::error::Error>> {
        self.proxy_factory.create(proxy_string, &self.init_data.properties)
    }

    pub fn property_to_proxy(&mut self, property: &str) -> Result<Proxy, Box<dyn std::error::Error>> {
        match self.init_data.properties.get(property) {
            Some(value) => {
                self.proxy_factory.create(value, &self.init_data.properties)
            }
            None => {
                Err(Box::new(PropertyError::new(property)))
            }
        }
    }
}

pub fn initialize(config_file: &str) -> Result<Communicator, Box<dyn std::error::Error>> {
    let mut init_data = InitializationData::new();
    init_data.properties.load(config_file).unwrap();
    let proxy_factory = ProxyFactory::new(&init_data.properties)?;
    Ok(Communicator {
        init_data,
        proxy_factory
    })
}