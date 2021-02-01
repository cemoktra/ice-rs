use async_trait::async_trait;

#[async_trait]
pub trait Transport {    
    async fn read(&mut self) -> tokio::io::Result<&[u8]>;
    async fn write(&mut self, buf: &mut [u8]) -> tokio::io::Result<usize>;

//     async fn validate_connection(&mut self) -> Result<(), Box<dyn std::error::Error>>
//     {
//         let header = Header::new(0, 14);
//         let mut bytes = header.to_bytes()?;
//         let written = self.write(&mut bytes).await?;
//         if written != header.message_size as usize {
//             return Err(Box::new(ProtocolError::new("TCP: Could not validate connection")))
//         }

//         Ok(())
//     }

//     async fn close_connection(&mut self) -> Result<(), Box<dyn std::error::Error>>
//     {
//         let header = Header::new(4, 14);
//         let mut bytes = header.to_bytes()?;
//         let written = self.write(&mut bytes).await?;
//         if written != header.message_size as usize {
//             return Err(Box::new(ProtocolError::new("TCP: Could not validate connection")))
//         }

//         Ok(())
//     }
}
