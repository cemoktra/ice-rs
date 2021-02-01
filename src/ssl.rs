use async_trait::async_trait;
use std::io::prelude::*;
use tokio::{io::AsyncReadExt, net::TcpStream};
use tokio_openssl::SslStream;
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use openssl::x509::*;
use openssl::pkcs12::*;
use openssl::pkey::*;
use std::fs::File;
use std::path::Path;
use tokio::io::AsyncWriteExt;

use crate::transport::Transport;
use crate::errors::*;
use crate::properties::Properties;

pub struct SslTransport {
    stream: SslStream<TcpStream>,
    buffer: Vec<u8>
}

impl Drop for SslTransport {
    fn drop(&mut self) {
        // self.close_connection().expect("Could not drop SslConnection");
    }
}

fn read_file(file_path: &Path) -> Result<Vec<u8>, Box<dyn std::error::Error + Sync + Send>> {
    let mut buffer = vec![];
    let mut file = File::open(file_path)?;
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn read_cert(file_path: &Path) -> Result<X509, Box<dyn std::error::Error + Sync + Send>> {
    let content = read_file(file_path)?;
    match X509::from_pem(&content) {
        Ok(cert) => Ok(cert),
        _ => Err(Box::new(ProtocolError::new("SSL: Could not read certificate")))
    }
}

fn read_key(file_path: &Path) -> Result<PKey<Private>, Box<dyn std::error::Error + Sync + Send>> {
    let content = read_file(file_path)?;
    match PKey::private_key_from_pem(&content) {
        Ok(cert) => Ok(cert),
        _ => Err(Box::new(ProtocolError::new("SSL: Could not read private key")))
    }
}

fn read_pkcs12(file_path: &Path, password: &str) -> Result<ParsedPkcs12, Box<dyn std::error::Error + Sync + Send>> {
    let content = read_file(file_path)?;
    match Pkcs12::from_der(&content) {
        Ok(pkcs12) => {
            match pkcs12.parse(password) {
                Ok(parsed) => Ok(parsed),
                _ => Err(Box::new(ProtocolError::new("SSL: Could not parse pkcs12")))
            }
        },
        _ => Err(Box::new(ProtocolError::new("SSL: Could not read pkcs12")))
    }
}

impl SslTransport {
    pub async fn new(address: &str, properties: &Properties) -> Result<SslTransport, Box<dyn std::error::Error + Sync + Send>>
    {
        let mut builder = SslConnector::builder(SslMethod::tls())?;
        let mut store_builder = store::X509StoreBuilder::new()?;
        let ssl_dir = Path::new(properties.get("IceSSL.DefaultDir").ok_or(Box::new(PropertyError::new("IceSSL.DefaultDir")))?);

        // TODO: this needs to support all kind of different key files

        // handle CA
        let mut ca_file = "";
        if properties.has("IceSSL.CAs") {
            ca_file = properties.get("IceSSL.CAs").ok_or(Box::new(PropertyError::new("IceSSL.CAs")))?;
        } else if properties.has("IceSSL.CertAuthFile") {
            ca_file = properties.get("IceSSL.CertAuthFile").ok_or(Box::new(PropertyError::new("IceSSL.CertAUthFile")))?;
        }
        let ca_path = ssl_dir.join(ca_file);
        let ca = read_cert(&ca_path)?;
        store_builder.add_cert(ca)?;
        let store = store_builder.build();
        builder.set_verify_cert_store(store)?;

        let verify_peer = properties.get("IceSSL.VerifyPeer").ok_or(Box::new(PropertyError::new("IceSSL.VerifyPeer")))?.parse::<u8>()?;
        match verify_peer {
            0 => builder.set_verify(SslVerifyMode::NONE),
            _ => builder.set_verify(SslVerifyMode::PEER)
        }


        // hanndle client
        let cert_file = properties.get("IceSSL.CertFile").ok_or(Box::new(PropertyError::new("IceSSL.CertFile")))?;
        let cert_path = ssl_dir.join(cert_file);
        if cert_path.extension().unwrap() == "p12" {
            let password = properties.get("IceSSL.Password").ok_or(Box::new(PropertyError::new("IceSSL.Password")))?;
            let cert = read_pkcs12(&cert_path, password)?;
            builder.set_certificate(&cert.cert)?;
            builder.set_private_key(&cert.pkey)?;
        } else {
            let cert = read_cert(&cert_path)?;
            builder.set_certificate(&cert)?;

            let key_file = properties.get("IceSSL.KeyFile").ok_or(Box::new(PropertyError::new("IceSSL.KeyFile")))?;
            let key_path = ssl_dir.join(key_file);
            let key = read_key(&key_path)?;
            builder.set_private_key(&key)?;
        }

        let connector = builder.build();
        let stream = TcpStream::connect(address).await?;
        let split = address.split(":").collect::<Vec<&str>>();
        let _host = split.first().unwrap();

        Ok(SslTransport {
            stream: SslStream::new(connector.configure()?.into_ssl(address)?, stream)?,
            buffer: vec![0; 4096]
        })
    }
}

#[async_trait]
impl Transport for SslTransport {
    async fn read(&mut self) -> tokio::io::Result<&[u8]> {
        let bytes = self.stream.read_buf(&mut self.buffer).await?;
        Ok(&self.buffer[0..bytes])
    }

    async fn write(&mut self, buf: &mut [u8]) -> tokio::io::Result<usize>
    {
        self.stream.write(buf).await
    }
}