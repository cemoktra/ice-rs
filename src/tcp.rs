use async_trait::async_trait;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

use crate::transport::Transport;

pub struct TcpTransport {
    stream: TcpStream,
    buffer: Vec<u8>
}

impl Drop for TcpTransport {
    fn drop(&mut self) {
        // self.close_connection().await.expect("Could not drop TcpConnection");
    }
}

impl TcpTransport {
    pub async fn new(address: &str) -> Result<TcpTransport, Box<dyn std::error::Error + Sync + Send>>
    {
        Ok(TcpTransport {
            stream: TcpStream::connect(address).await?,
            buffer: vec![0; 4096]
        })
    }
}

#[async_trait]
impl Transport for TcpTransport {
    async fn read(&mut self) -> tokio::io::Result<&[u8]> {
        let bytes = self.stream.read(&mut self.buffer).await?;
        Ok(&self.buffer[0..bytes])
    }

    async fn write(&mut self, buf: &mut [u8]) -> tokio::io::Result<usize>
    {
        self.stream.write(buf).await
    }
}