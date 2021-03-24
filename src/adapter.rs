use crate::errors::*;
use crate::proxy_parser::*;
use crate::iceobject::*;
use crate::protocol::*;
use crate::encoding::*;
use std::collections::BTreeMap;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};


pub struct Adapter {
    endpoint: DirectProxyData,
    objects: BTreeMap<String, Box<dyn IceObjectServer + Send + Sync>>
}

impl Adapter {
    pub fn with_endpoint(name: &str, endpoint: &str) -> Result<Adapter, Box<dyn std::error::Error + Sync + Send>> {
        let endpoint = parse_proxy_string(&format!("{}:{}", name, endpoint))?;
        let endpoint = match endpoint {
            ProxyStringType::DirectProxy(endpoint) => {
                endpoint
            }
            _ => {
                return Err(Box::new(ProtocolError::new("Direct proxy required for endpoint")))
            }
        };

        Ok(Adapter{
            endpoint,
            objects: BTreeMap::new()
        })
    }

    pub fn add(&mut self, ident: &str, object: Box<dyn IceObjectServer + Send + Sync>) {
        self.objects.insert(String::from(ident), object);
    }

    pub async fn activate(&mut self) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        let listener = match &self.endpoint.endpoint {
            EndPointType::TCP(data) => {
                TcpListener::bind(format!("{}:{}", data.host, data.port)).await?
            },
            _ => {
                return Err(Box::new(ProtocolError::new("Direct proxy required for endpoint")))
            }
        };

        loop {
            let (mut socket, _) = listener.accept().await?;
            self.handle_socket(&mut socket).await?;
        }
    }

    pub async fn handle_socket(&mut self, stream: &mut TcpStream) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        let mut buffer = [0u8; 4096];

        let header = Header::new(3, 14);
        let mut bytes = header.to_bytes()?;
        stream.write(&mut bytes).await?;
        loop {
            let bytes = stream.read(&mut buffer).await?;
            let mut read = 0;
            let header = Header::from_bytes(&buffer[0..bytes], &mut read)?;
            match header.message_type {
                0 => {
                    let req = RequestData::from_bytes(&buffer[read as usize..bytes], &mut read)?;
                    let reply = if let Some(object) = self.objects.get_mut(&req.id.name) {
                        match object.handle_request(&req).await {
                            Ok(reply) => reply,
                            Err(e) => {
                                ReplyData {
                                    request_id: req.request_id,
                                    status: 1,
                                    body: Encapsulation::from(e.to_string().as_bytes().to_vec())
                                }
                            }
                        }
                    } else {
                        ReplyData {
                            request_id: req.request_id,
                            status: 1,
                            body: Encapsulation::from(String::from("Object not found").as_bytes().to_vec())
                        }
                    };
        
                    let header = Header::new(2, reply.body.size + 19);
                    let mut return_buffer = header.to_bytes()?;
                    return_buffer.extend(reply.to_bytes()?);
                    stream.write(&mut return_buffer).await?;                
                }
                4 => {
                    return Ok(())
                }
                _ => {
                    return Err(Box::new(ProtocolError::new("Unsupported message type")))
                }
            }
            
        }
    }
}