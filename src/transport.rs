use tokio::io::{AsyncRead, AsyncWrite};

pub trait Transport: AsyncRead + AsyncWrite + Send {
    fn transport_type(&self) -> String;
}
