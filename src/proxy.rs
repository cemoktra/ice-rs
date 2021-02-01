use std::{collections::HashMap, hash::Hash};


use task::JoinHandle;
use tokio::task;

use crate::{errors::{ProtocolError, UserError}, protocol::{Header, MessageType}, transport::Transport};
use crate::protocol::{ReplyData, RequestData, Identity, Encapsulation};
use crate::encoding::{ToBytes, FromBytes};

#[derive(Parser)]
#[grammar = "proxystring.pest"]
pub struct ProxyParser;

pub struct Proxy {
    pub stream: Box<dyn Transport + Send + Sync>,
    pub request_id: i32,
    pub ident: String,
    pub host: String,
    pub port: i32,
    pub context: Option<HashMap<String, String>>,
    pub handle: Option<JoinHandle<Result<(), Box<dyn std::error::Error + Sync + Send>>>>
}


impl Drop for Proxy {
    fn drop(&mut self) {
        // self.close_connection().await.expect("Could not drop TcpConnection");
    }
}

impl Proxy {
    pub fn new(stream: Box<dyn Transport + Send + Sync>, ident: &str, host: &str, port: i32, context: Option<HashMap<String, String>>) -> Proxy {
        Proxy {
            stream,
            request_id: 0,
            ident: String::from(ident),
            host: String::from(host),
            port,
            context: context,
            handle: None
        }
    }

    // pub fn read_thread(&mut self) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    //     let handle = task::spawn(async move {
    //         loop {
    //             match self.read_message::<ProtocolError>().await {
    //                 Ok(message) => {

    //                 },
    //                 Err(e) => return Err(e)
    //             };
    //         }
    //         Ok(())
    //     });
    //     todo!();
    // }

    // pub fn ice_context(&mut self, context: HashMap<String, String>) -> Proxy {
    //     Proxy {
    //         // TODO: should not clone, but create new
    //         stream: self.stream.clone(),
    //         request_id: self.request_id,
    //         ident: self.ident.clone(),
    //         host: self.host.clone(),
    //         port: self.port,
    //         context: Some(context)
    //     }
    // }

    pub fn create_request(&mut self, identity_name: &str, operation: &str, mode: u8, params: &Encapsulation, context: Option<HashMap<String, String>>) -> RequestData {
        let context = match context {
            Some(context) => context,
            None => {
                match self.context.as_ref() {
                    Some(context) => context.clone(),
                    None => HashMap::new()
                }
            }
        };
        self.request_id = self.request_id + 1;
        RequestData {
            request_id: self.request_id,
            id: Identity::new(identity_name),
            facet: Vec::new(),
            operation: String::from(operation),
            mode: mode,
            context: context,
            params: params.clone()
        }
    }

    async fn send_request(&mut self, request: &RequestData) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        let req_bytes = request.to_bytes()?;
        let header = Header::new(0, 14 + req_bytes.len() as i32);
        let mut bytes = header.to_bytes()?;
        bytes.extend(req_bytes);

        let written = self.stream.write(&mut bytes).await?;
        if written != header.message_size as usize {
            return Err(Box::new(ProtocolError::new(&format!("TCP: Error writing request {}", request.request_id))))
        }
        Ok(())
    }

    async fn read_response<T: 'static + std::fmt::Debug + std::fmt::Display + FromBytes + Send + Sync>(&mut self) -> Result<ReplyData, Box<dyn std::error::Error + Sync + Send>> {
        let message = self.read_message::<T>().await?;
        match message {
            MessageType::Reply(_header, reply) => {
                match reply.status {
                    1 => {
                        let mut read = 0;
                        Err(Box::new(UserError {
                            exception: T::from_bytes(&reply.body.data, &mut read)?
                        }))
                    }
                    _ => Ok(reply)
                }
            },
            _ => Err(Box::new(ProtocolError::new(&format!("Unsupported message type: {:?}", message))))
        }
    }

    pub async fn read_message<T: 'static + std::fmt::Debug + std::fmt::Display + FromBytes + Send + Sync>(&mut self) -> Result<MessageType, Box<dyn std::error::Error + Sync + Send>> {
        let buffer = self.stream.read().await?;
        let mut read: i32 = 0;
        let bytes = buffer.len();
        let header = Header::from_bytes(&buffer[read as usize..14], &mut read)?;

        match header.message_type {
            2 => {
                let reply = ReplyData::from_bytes(&buffer[read as usize..bytes as usize], &mut read)?;
                Ok(MessageType::Reply(header, reply))
            }
            3 => Ok(MessageType::ValidateConnection(header)),
            _ => Err(Box::new(ProtocolError::new(&format!("TCP: Unsuppored reply message type: {}", header.message_type))))
        }
    }

    pub async fn make_request<T: 'static + std::fmt::Debug + std::fmt::Display + FromBytes + Send + Sync>(&mut self, request: &RequestData) -> Result<ReplyData, Box<dyn std::error::Error + Sync + Send>>
    {
        self.send_request(request).await?;
        self.read_response::<T>().await
    }
}
// Ice::ObjectNotFoundException