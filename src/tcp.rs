use tokio::{io::{AsyncRead, AsyncWrite}, net::TcpStream};

use crate::transport::Transport;

pub struct TcpTransport {
    stream: TcpStream
}

impl TcpTransport {
    pub async fn new(address: &str) -> Result<TcpTransport, Box<dyn std::error::Error + Sync + Send>>
    {
        Ok(TcpTransport {
            stream: TcpStream::connect(address).await?
        })
    }
}

impl AsyncWrite for TcpTransport {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        std::pin::Pin::new(&mut self.get_mut().stream).poll_write(cx, buf)
    }

    fn poll_flush(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), std::io::Error>> {
        std::pin::Pin::new(&mut self.get_mut().stream).poll_flush(cx)
    }

    fn poll_shutdown(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), std::io::Error>> {
        std::pin::Pin::new(&mut self.get_mut().stream).poll_shutdown(cx)
    }
}

impl AsyncRead for TcpTransport {
    fn poll_read(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>, buf: &mut tokio::io::ReadBuf<'_>) -> std::task::Poll<std::io::Result<()>> {
        std::pin::Pin::new(&mut self.get_mut().stream).poll_read(cx, buf)
    }
}

impl Transport for TcpTransport {
    fn transport_type(&self) -> String {
        return String::from("tcp");
    }
}