use std::{collections::HashMap, hash::Hash};
use std::sync::Arc;

use task::JoinHandle;
use tokio::{io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf}, sync::Mutex, task};

use crate::{errors::{ProtocolError, UserError}, protocol::{Header, MessageType}, proxy_factory::ProxyFactory, proxy_parser::{ProxyStringType, parse_proxy_string}, transport::Transport};
use crate::protocol::{ReplyData, RequestData, Identity, Encapsulation};
use crate::encoding::{ToBytes, FromBytes};

#[derive(Parser)]
#[grammar = "proxystring.pest"]
pub struct ProxyParser;

pub struct Proxy {
    pub write: WriteHalf<Box<dyn Transport + Send + Sync + Unpin>>,
    pub request_id: i32,
    pub ident: String,
    pub host: String,
    pub port: i32,
    pub context: Option<HashMap<String, String>>,
    pub handle: Option<JoinHandle<Result<(), Box<dyn std::error::Error + Sync + Send>>>>,
    pub message_queue: Arc<Mutex<Vec<MessageType>>>,
    pub stream_type: String
}


impl Drop for Proxy {
    fn drop(&mut self) {
        tokio::task::block_in_place(|| {
            futures::executor::block_on(async {
                self.close_connection().await
            })
        }).expect("Could not close connection");
        match &self.handle {
            Some(handle) => handle.abort(),
            None => {}
        };
    }
}

impl Proxy {
    async fn read_thread(mut rx: ReadHalf<Box<dyn Transport + Send + Sync + Unpin>>, message_queue: Arc<Mutex<Vec<MessageType>>>) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        let mut buffer = vec![0; 2048];
        loop {
            let bytes = rx.read(&mut buffer).await?;
            let mut read: i32 = 0;
            let header = Header::from_bytes(&buffer[read as usize..bytes], &mut read)?;

            let message = match header.message_type {
                2 => {
                    let reply = ReplyData::from_bytes(&buffer[read as usize..bytes as usize], &mut read)?;                    
                    MessageType::Reply(header, reply)
                }
                3 => {
                    MessageType::ValidateConnection(header)
                },
                _ => return Err(Box::new(ProtocolError::new(&format!("TCP: Unsuppored reply message type: {}", header.message_type))))
            };

            {
                let mut lock = message_queue.lock().await;
                lock.push(message);
            }

            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }

    pub fn new(stream: Box<dyn Transport + Send + Sync + Unpin>, ident: &str, host: &str, port: i32, context: Option<HashMap<String, String>>) -> Proxy {
        let stream_type = stream.transport_type();
        let (rx, tx) = tokio::io::split(stream);
        let mut proxy = Proxy {
            write: tx,
            request_id: 0,
            ident: String::from(ident),
            host: String::from(host),
            port,
            context: context,
            handle: None,
            message_queue: Arc::new(Mutex::new(Vec::new())),
            stream_type
        };
        let message_queue = proxy.message_queue.clone();
        proxy.handle = Some(task::spawn(async move {
            Proxy::read_thread(rx, message_queue).await
        }));

        proxy
    }

    async fn close_connection(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    {
        let header = Header::new(4, 14);
        let mut bytes = header.to_bytes()?;
        let written = self.write.write(&mut bytes).await?;
        if written != header.message_size as usize {
            return Err(Box::new(ProtocolError::new("TCP: Could not validate connection")))
        }

        Ok(())
    }

    pub async fn ice_context(&mut self, context: HashMap<String, String>) -> Result<Proxy, Box<dyn std::error::Error + Send + Sync>> {
        let init_data = crate::communicator::INITDATA.lock().unwrap();
        let proxy_string = format!("{}:{} -h {} -p {}", self.ident, self.stream_type, self.host, self.port);
        match parse_proxy_string(&proxy_string)? {
            ProxyStringType::DirectProxy(data) => {
                ProxyFactory::create_proxy(data, init_data.properties(), Some(context)).await
            }
            _ => {
                Err(Box::new(ProtocolError::new("ice_context() - could not create proxy")))
            }
        }
    }

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

        let written = self.write.write(&mut bytes).await?;
        if written != header.message_size as usize {
            return Err(Box::new(ProtocolError::new(&format!("TCP: Error writing request {}", request.request_id))))
        }
        Ok(())
    }

    pub async fn await_validate_connection_message(&mut self) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
        let timeout = std::time::Duration::from_secs(30); // TODO: read from ice config
        let now = std::time::Instant::now();

        loop {
            {
                let mut lock = self.message_queue.lock().await;
                let index = lock.iter().position(|i| {
                    match i {
                        MessageType::ValidateConnection(_) => true,
                        _ => false
                    }
                });
                match index {
                    Some(index) => {
                        lock.swap_remove(index);
                        break;
                    },
                    None => {}
                }
            }

            if now.elapsed() >= timeout {
                return Err(Box::new(ProtocolError::new("Timeout waiting for response")));
            }

            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        Ok(())
    }

    pub async fn await_reply_message(&mut self, request_id: i32) -> Result<MessageType, Box<dyn std::error::Error + Sync + Send>> {
        let timeout = std::time::Duration::from_secs(30); // TODO: read from ice config
        let now = std::time::Instant::now();

        loop {
            {
                let mut lock = self.message_queue.lock().await;
                let index = lock.iter().position(|i| {
                    match i {
                        MessageType::Reply(_, data) => {
                            if data.request_id == request_id {
                                true
                            } else {
                                false
                            }
                        },
                        _ => false
                    }
                });
                match index {
                    Some(index) => {
                        let result = lock.swap_remove(index);
                        return Ok(result)
                    },
                    None => {}
                }
            }

            if now.elapsed() >= timeout {
                return Err(Box::new(ProtocolError::new("Timeout waiting for response")));
            }

            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }

    async fn read_response<T: 'static + std::fmt::Debug + std::fmt::Display + FromBytes + Send + Sync>(&mut self, request_id: i32) -> Result<ReplyData, Box<dyn std::error::Error + Sync + Send>> {
        let message = self.await_reply_message(request_id).await?;
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

    pub async fn make_request<T: 'static + std::fmt::Debug + std::fmt::Display + FromBytes + Send + Sync>(&mut self, request: &RequestData) -> Result<ReplyData, Box<dyn std::error::Error + Sync + Send>>
    {
        self.send_request(request).await?;
        self.read_response::<T>(request.request_id).await
    }
}