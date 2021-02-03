use tokio::io::{AsyncRead, AsyncWrite};

pub trait Transport: AsyncRead + AsyncWrite + Send {}
