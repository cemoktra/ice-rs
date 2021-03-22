use std::sync::Mutex;

use crate::{proxy::Proxy, proxy_factory::ProxyFactory};
use crate::initdata::InitializationData;
use crate::errors::PropertyError;
use crate::adapter::*;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref INITDATA: Mutex<InitializationData> = Mutex::new(InitializationData::new());
}

/// The Communicator is a basic object in ZeroC Ice. Currently
/// this is more a stub that does dummy initialization.
pub struct Communicator {
    proxy_factory: ProxyFactory
}

impl Communicator {
    pub async fn new() -> Result<Communicator, Box<dyn std::error::Error + Sync + Send>> {
        let init_data = INITDATA.lock().unwrap();
        let proxy_factory = ProxyFactory::new(init_data.properties()).await?;
        Ok(Communicator {
            proxy_factory
        })
    }

    pub async fn string_to_proxy(&mut self, proxy_string: &str) -> Result<Proxy, Box<dyn std::error::Error + Sync + Send>> {
        let init_data = INITDATA.lock().unwrap();
        self.proxy_factory.create(proxy_string, init_data.properties()).await
    }

    pub async fn property_to_proxy(&mut self, property: &str) -> Result<Proxy, Box<dyn std::error::Error + Sync + Send>> {
        let init_data = INITDATA.lock().unwrap();
        let properties = init_data.properties();
        match properties.get(property) {
            Some(value) => {
                self.proxy_factory.create(value, &properties).await
            }
            None => {
                Err(Box::new(PropertyError::new(property)))
            }
        }
    }

    pub async fn create_object_adapter_with_endpoint(&self, name: &str, endpoint: &str) -> Result<Adapter, Box<dyn std::error::Error + Sync + Send>> {
        Adapter::with_endpoint(name, endpoint)
    }
}

pub async fn initialize(config_file: &str) -> Result<Communicator, Box<dyn std::error::Error + Sync + Send>> {
    let mut init_data = INITDATA.lock().unwrap();
    let properties = init_data.properties_as_mut();
    properties.load(config_file).unwrap();
    let proxy_factory = ProxyFactory::new(&properties).await?;
    Ok(Communicator {
        proxy_factory
    })
}