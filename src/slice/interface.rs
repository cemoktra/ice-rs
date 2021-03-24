use crate::slice::function::Function;
use quote::{__private::TokenStream, format_ident, quote};


#[derive(Clone, Debug)]
pub struct Interface {
    pub id: TokenStream,
    pub ice_id: String,
    pub functions: Vec<Function>
}

impl Interface {
    pub fn empty() -> Interface {
        Interface {
            id: TokenStream::new(),
            ice_id: String::from(""),
            functions: Vec::new()
        }
    }

    pub fn add_function(&mut self, function: Function) {
        self.functions.push(function);
    }

    pub fn generate(&self, mod_path: &str) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let mut decl_tokens = TokenStream::new();
        for function in &self.functions {
            let token = function.generate_decl()?;
            decl_tokens = quote! {
                #decl_tokens
                #token
            };
        }
        let mut impl_tokens = TokenStream::new();
        for function in &self.functions {
            let token = function.generate_impl()?;
            impl_tokens = quote! {
                #impl_tokens
                #token
            };
        }
        let mut server_decl_tokens = TokenStream::new();
        for function in &self.functions {
            let token = function.generate_server_decl()?;
            server_decl_tokens = quote! {
                #server_decl_tokens
                #token
            };
        }
        let mut server_handler_tokens = TokenStream::new();
        for function in &self.functions {
            let token = function.generate_server_handler()?;
            server_handler_tokens = quote! {
                #server_handler_tokens
                #token
            };
        }

        let id_token = &self.id;
        let id_proxy_token = format_ident!("{}Prx", self.id.to_string());
        let id_server_trait_token = format_ident!("{}I", self.id.to_string());
        let id_server_token = format_ident!("{}Server", self.id.to_string());
        let type_id_token = format!("{}::{}", mod_path, self.ice_id);
        Ok(quote! {
            #[async_trait]
            pub trait #id_token : IceObject {
                #decl_tokens
            }

            #[async_trait]
            pub trait #id_server_trait_token {
                #server_decl_tokens
            }

            pub struct #id_server_token {
                server_impl: Box<dyn #id_server_trait_token + Send + Sync>
            }

            impl #id_server_token {
                #[allow(dead_code)]
                pub fn new(server_impl: Box<dyn #id_server_trait_token + Send + Sync>) -> #id_server_token {
                    #id_server_token {
                        server_impl
                    }
                }

                async fn ice_is_a(&self, param: &str) -> bool {
                    param == #type_id_token
                }
                // TODO: ice_ids etc...
            }

            #[async_trait]
            impl IceObjectServer for #id_server_token {
                async fn handle_request(&mut self, request: &RequestData) -> Result<ReplyData, Box<dyn std::error::Error + Sync + Send>> {
                    match request.operation.as_ref() {
                        "ice_isA" => {
                            let mut read = 0;
                            let param = String::from_bytes(&request.params.data, &mut read)?;
                            Ok(ReplyData {
                                request_id: request.request_id,
                                status: 0,
                                body: Encapsulation::from(self.ice_is_a(&param).await.to_bytes()?)
                            })
                        },
                        #server_handler_tokens
                        _ => Err(Box::new(ProtocolError::new("Operation not found")))
                    }
                }
            }

            pub struct #id_proxy_token {
                pub proxy: Proxy
            }

            #[async_trait]
            impl IceObject for #id_proxy_token {
                async fn ice_ping(&mut self) -> Result<(), Box<dyn std::error::Error + Sync + Send>>
                {
                    self.proxy.dispatch::<ProtocolError>(&String::from("ice_ping"), 1, &Encapsulation::empty(), None).await?;
                    Ok(())
                }

                async fn ice_is_a(&mut self) -> Result<bool, Box<dyn std::error::Error + Sync + Send>> {
                    let reply = self.proxy.dispatch::<ProtocolError>(&String::from("ice_isA"), 1, &Encapsulation::from(String::from(#type_id_token).to_bytes()?), None).await?;
                    let mut read_bytes: i32 = 0;
                    bool::from_bytes(&reply.body.data, &mut read_bytes)
                }

                async fn ice_id(&mut self) -> Result<String, Box<dyn std::error::Error + Sync + Send>>
                {
                    let reply = self.proxy.dispatch::<ProtocolError>(&String::from("ice_id"), 1, &Encapsulation::empty(), None).await?;
                    let mut read_bytes: i32 = 0;
                    String::from_bytes(&reply.body.data, &mut read_bytes)
                }

                async fn ice_ids(&mut self) -> Result<Vec<String>, Box<dyn std::error::Error + Sync + Send>>
                {
                    let reply = self.proxy.dispatch::<ProtocolError>(&String::from("ice_ids"), 1, &Encapsulation::empty(), None).await?;
                    let mut read_bytes: i32 = 0;
                    Vec::from_bytes(&reply.body.data, &mut read_bytes)
                }
            }

            #[async_trait]
            impl #id_token for #id_proxy_token {
                #impl_tokens
            }

            impl #id_proxy_token {
                #[allow(dead_code)]
                pub async fn unchecked_cast(proxy: Proxy) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
                    Ok(Self {
                        proxy: proxy,
                    })
                }

                #[allow(dead_code)]
                pub async fn checked_cast(proxy: Proxy) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
                    let mut my_proxy = Self::unchecked_cast(proxy).await?;
            
                    if !my_proxy.ice_is_a().await? {
                        return Err(Box::new(ProtocolError::new("ice_is_a() failed")));
                    }
                    Ok(my_proxy)
                }
            }
        })
    }
}
