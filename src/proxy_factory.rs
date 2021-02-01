use crate::{errors::ProtocolError, locator::Locator, properties::Properties, protocol::EndPointType, proxy::Proxy, proxy_parser::{DirectProxyData, ProxyStringType, parse_proxy_string}, ssl::SslTransport, tcp::TcpTransport};

pub struct ProxyFactory {
    locator: Option<Locator>
}

impl ProxyFactory {
    async fn create_proxy(proxy_data: DirectProxyData, properties: &Properties) -> Result<Proxy, Box<dyn std::error::Error + Sync + Send>> {
        let mut proxy = match proxy_data.endpoint {
            EndPointType::TCP(endpoint) => {
                Proxy::new(
                    Box::new(TcpTransport::new(&format!("{}:{}", endpoint.host, endpoint.port)).await?),
                    &proxy_data.ident,
                    &endpoint.host,
                    endpoint.port,
                    None
                )
            }
            EndPointType::SSL(endpoint) => {
                Proxy::new(
                    Box::new(SslTransport::new(&format!("{}:{}", endpoint.host, endpoint.port), properties).await?),
                    &proxy_data.ident,
                    &endpoint.host,
                    endpoint.port,
                    None
                )
            }
            _ => return Err(Box::new(ProtocolError::new(&format!("Error creating proxy"))))
        };

        proxy.read_message::<ProtocolError>().await?;

        Ok(proxy)
    }

    pub async fn new(properties: &Properties) -> Result<ProxyFactory, Box<dyn std::error::Error + Sync + Send>> {
        Ok(ProxyFactory {
            locator: match properties.get("Ice.Default.Locator") {
                Some(locator_proxy) => {
                    match parse_proxy_string(locator_proxy) {
                        Ok(proxy_type) => {
                            match proxy_type {
                                ProxyStringType::DirectProxy(data) => {
                                    Some(Locator::from(ProxyFactory::create_proxy(data, properties).await?))
                                }
                                _ => None
                            }
                        },
                        _ => None
                    }
                },
                _ => None
            }
        })
    }

    pub async fn create(&mut self, proxy_string: &str, properties: &Properties) -> Result<Proxy, Box<dyn std::error::Error + Sync + Send>> {
        match parse_proxy_string(proxy_string)? {
            ProxyStringType::DirectProxy(data) => {
                ProxyFactory::create_proxy(data, properties).await
            }
            ProxyStringType::IndirectProxy(data) => {
                match self.locator.as_mut() {
                    Some(locator) => {
                        let data = locator.locate(data).await?;
                        ProxyFactory::create_proxy(data, properties).await
                    }
                    _ => Err(Box::new(ProtocolError::new(&format!("No locator set up to resolve indirect proxy"))))
                }
            }
        }
    }
}