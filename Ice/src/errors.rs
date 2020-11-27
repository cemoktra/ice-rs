#[derive(Debug)]
pub enum Error {
    CannotDeserialize,
    WrongProtocolMagic,
    TcpCannotConnect,
    UnknownMessageType,
    // CannotSerialize,
    MessageWriteError,
}  