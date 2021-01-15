use std::collections::HashMap;


use crate::{errors::ProtocolError, properties::Properties, protocol::{Encapsulation, Identity, LocatorResult, RequestData}};
use crate::encoding::{ToBytes,FromBytes};
use crate::proxy::Proxy;

pub struct Locator {
    proxy: Proxy,
    request_id: i32
}

impl Locator {
    pub fn new(proxy_string: &str, properties: &Properties) -> Result<Locator, Box<dyn std::error::Error>> {
        Ok(Locator {
            proxy: Proxy::new(proxy_string, properties)?,
            request_id: 0
        })
    }

    pub fn find_object_by_id(&mut self, req: &str) -> Result<LocatorResult, Box<dyn std::error::Error>> {
        self.request_id = self.request_id + 1;
        let mut bytes = req.to_bytes()?;
        bytes.push(0);
        let req_data = RequestData {
            request_id: self.request_id,
            id: Identity::new(&self.proxy.ident),
            facet: vec![],
            operation: String::from("findObjectById"),
            mode: 1,
            context: HashMap::new(),
            params: Encapsulation::from(bytes)
        };
        let reply = self.proxy.make_request::<ProtocolError>(&req_data)?;

        let mut read = 0;
        LocatorResult::from_bytes(&reply.body.data[read as usize..reply.body.data.len()], &mut read)
    }

    pub fn find_adapter_by_id(&mut self, req: &str) -> Result<LocatorResult, Box<dyn std::error::Error>> {
        self.request_id = self.request_id + 1;
        let bytes = req.to_bytes()?;
        let req_data = RequestData {
            request_id: self.request_id,
            id: Identity::new(&self.proxy.ident),
            facet: vec![],
            operation: String::from("findAdapterById"),
            mode: 1,
            context: HashMap::new(),
            params: Encapsulation::from(bytes)
        };
        let reply = self.proxy.make_request::<ProtocolError>(&req_data)?;
        
        let mut read = 0;
        LocatorResult::from_bytes(&reply.body.data[read as usize..reply.body.data.len()], &mut read)
    }
}