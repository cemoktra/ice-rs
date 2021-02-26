
use tokio::{io::{AsyncRead, AsyncWrite}, net::TcpStream};
use tokio_openssl::SslStream;
use openssl::{ssl::{SslConnector, SslConnectorBuilder, SslMethod, SslVerifyMode}};
use openssl::x509::*;
use std::path::Path;

use crate::transport::Transport;
use crate::errors::*;
use crate::properties::Properties;

use crate::ssltools::*;

pub struct SslTransport {
    stream: SslStream<TcpStream>
}

impl SslTransport {
    pub async fn new(address: &str, properties: &Properties) -> Result<SslTransport, Box<dyn std::error::Error + Sync + Send>>
    {
        let mut builder = SslConnector::builder(SslMethod::tls())?;
        let ssl_dir = Path::new(properties.get("IceSSL.DefaultDir").ok_or(Box::new(PropertyError::new("IceSSL.DefaultDir")))?);

        configure_ca(&ssl_dir, properties, &mut builder)?;
        configure_client_certs(&ssl_dir, properties, &mut builder)?;
        configure_peer_verification(properties, &mut builder)?;
        configure_ciphers(properties, &mut builder)?;
        configure_protocol_versions(properties, &mut builder)?;

        // connect
        let connector = builder.build();
        let stream = TcpStream::connect(address).await?;
        let mut stream = SslStream::new(connector.configure()?.into_ssl(address)?, stream)?;
        std::pin::Pin::new(&mut stream).connect().await.unwrap();
        Ok(SslTransport {
            stream
        })
    }
}
impl AsyncWrite for SslTransport {
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

impl AsyncRead for SslTransport {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        std::pin::Pin::new(&mut self.get_mut().stream).poll_read(cx, buf)
    }
}

impl Transport for SslTransport {
    fn transport_type(&self) -> String {
        return String::from("ssl");
    }
}


fn configure_ca(ssl_dir: &Path, properties: &Properties, builder: &mut SslConnectorBuilder) -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    let ca = match properties.get("IceSSL.CAs") {
        Some(ca_file) => {
            // PEM
            read_pem(ca_file, ssl_dir)?.0.unwrap()
        }
        _ => {
            if let Some(ca_file) = properties.get("IceSSL.CertAuthFile") {
                // PEM [DEPRECATED]
                println!("[SSL] Use of deprecated property IceSSL.CertAuthFile");
                read_pem(ca_file, ssl_dir)?.0.unwrap()
            } else {
                return Ok(());
            }
        }
    };
    let mut store_builder = store::X509StoreBuilder::new()?;
    store_builder.add_cert(ca)?;
    let store = store_builder.build();
    builder.set_verify_cert_store(store)?;

    Ok(())
}

fn configure_client_certs(ssl_dir: &Path, properties: &Properties, builder: &mut SslConnectorBuilder) -> Result<(), Box<dyn std::error::Error + Sync + Send>>
{
    let (cert, pkey) = match properties.get("IceSSL.CertFile") {
        Some(cert_file) => {
            if let Some(key_file) = properties.get("IceSSL.KeyFile") {
                // PEM [DEPRECATED]
                println!("[SSL] Use of deprecated property IceSSL.KeyFile");
                let (cert, _) = read_pem(cert_file, ssl_dir)?;
                let (_, pkey) = read_pem(key_file, ssl_dir)?;
                (cert.unwrap(), pkey.unwrap())
            } else {
                // PKCS12
                let password = properties.get("IceSSL.Password").ok_or(Box::new(PropertyError::new("Use of IceSSL.CertFile requires IceSSL.Password to be set")))?;
                let pkcs12 = read_pkcs12(Path::new(cert_file), password, ssl_dir)?;
                (pkcs12.cert, pkcs12.pkey)
            }
        }
        _ => {
            return Ok(());
        }
    };
    builder.set_certificate(&cert)?;
    builder.set_private_key(&pkey)?;
    Ok(())
}

fn configure_peer_verification(properties: &Properties, builder: &mut SslConnectorBuilder) -> Result<(), Box<dyn std::error::Error + Sync + Send>>
{
    match properties.get("IceSSL.VerifyPeer").unwrap_or(&String::from("1")).parse::<u8>()? {
        0 => builder.set_verify(SslVerifyMode::NONE),
        _ => builder.set_verify(SslVerifyMode::PEER)
    }
    Ok(())
}

fn configure_ciphers(properties: &Properties, builder: &mut SslConnectorBuilder) -> Result<(), Box<dyn std::error::Error + Sync + Send>>
{
    if let Some(ciphers) =  properties.get("IceSSL.Ciphers") {
        builder.set_cipher_list(ciphers)?;
    }
    Ok(())
}

fn configure_protocol_versions(properties: &Properties, builder: &mut SslConnectorBuilder) -> Result<(), Box<dyn std::error::Error + Sync + Send>>
{
    let mut min_proto = None;
    let mut max_proto = None;
    if let Some(protocols) = properties.get("IceSSL.Protocols") {
        for protocol in protocols.split(",").collect::<Vec<&str>>() {
            if let Some(protocol) = parse_protocol(protocol) {
                max_proto = Some(max_protocol(protocol, max_proto));
                min_proto = Some(min_protocol(protocol, min_proto));
            }
        }
    }

    builder.set_min_proto_version(min_proto)?;
    builder.set_max_proto_version(max_proto)?;
    Ok(())
}
