#[derive(Debug)]
pub enum Error {
    CannotDeserialize,
    WrongProtocolMagic,
    TcpCannotConnect,
    UnknownMessageType,
    MessageWriteError,
    CannotResolveProxy,
    NotImplemented,
}  