#[derive(Debug)]
pub enum Error {
    EncodingError,
    DecodingError,
    TcpError,
    ProtocolError,
    ParsingError,
    Unexpected
}  