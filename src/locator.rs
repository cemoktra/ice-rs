use std::collections::HashMap;


use crate::{errors::ProtocolError, protocol::{Encapsulation, EndPointType, Identity, LocatorResult, RequestData}, proxy_parser::{DirectProxyData, IndirectProxyData}};
use crate::encoding::{ToBytes,FromBytes};
use crate::proxy::Proxy;

pub struct Locator {
    proxy: Proxy,
    request_id: i32
}

impl Locator {
    pub fn from(proxy: Proxy) -> Locator {
        Locator {
            proxy: proxy,
            request_id: 0
        }
    }

    pub async fn locate(&mut self, proxy_data: IndirectProxyData) -> Result<DirectProxyData, Box<dyn std::error::Error + Sync + Send>> {
        match proxy_data.adapter {
            Some(adapter) => {
                let result = self.find_adapter_by_id(&adapter).await?;
                Ok(DirectProxyData {
                    ident: result.proxy_data.id,
                    endpoint: result.endpoint
                })
            }
            None => {
                let obj_result = self.find_object_by_id(&proxy_data.ident).await?;
                match obj_result.endpoint {
                    EndPointType::WellKnownObject(object) => {
                        let adapter_result = self.find_adapter_by_id(&object).await?;
                        Ok(DirectProxyData {
                            ident: obj_result.proxy_data.id,
                            endpoint: adapter_result.endpoint
                        })
                    }
                    _ => Ok(DirectProxyData {
                        ident: obj_result.proxy_data.id,
                        endpoint: obj_result.endpoint
                    })
                }

            }
        }
    }

    pub async fn find_object_by_id(&mut self, req: &str) -> Result<LocatorResult, Box<dyn std::error::Error + Sync + Send>> {
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
        let reply = self.proxy.make_request::<ProtocolError>(&req_data).await?;

        let mut read = 0;
        LocatorResult::from_bytes(&reply.body.data[read as usize..reply.body.data.len()], &mut read)
    }

    pub async fn find_adapter_by_id(&mut self, req: &str) -> Result<LocatorResult, Box<dyn std::error::Error + Sync + Send>> {
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
        let reply = self.proxy.make_request::<ProtocolError>(&req_data).await?;

        let mut read = 0;
        LocatorResult::from_bytes(&reply.body.data[read as usize..reply.body.data.len()], &mut read)
    }
}