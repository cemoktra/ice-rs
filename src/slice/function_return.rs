use quote::{__private::TokenStream, format_ident, quote};

use super::types::IceType;

#[derive(Clone, Debug)]
pub struct FunctionReturn {
    pub r#type: IceType,
    pub return_proxy: bool
}

impl FunctionReturn {
    pub fn new(r#type: IceType) -> FunctionReturn {
        FunctionReturn {
            r#type: r#type,
            return_proxy: false
        }
    }

    pub fn empty() -> FunctionReturn {
        FunctionReturn {
            r#type: IceType::VoidType,
            return_proxy: false
        }
    }

    pub fn set_proxy(&mut self) {
        self.return_proxy = true;
    }

    pub fn token(&self) -> TokenStream {
        let token = self.r#type.token();
        if self.return_proxy {
            let typename = token.to_string();
            let ident = format_ident!("{}{}", typename, if self.return_proxy { "Prx"} else {""});
            quote! { #ident }
        } else {
            token
        }
    }

    pub fn return_token(&self) -> TokenStream {
        let return_token = self.token();
        match &self.r#type {
            IceType::VoidType => {
                quote! {
                    Ok(())
                }
            }
            IceType::Optional(type_name, _) => {
                let option_token = type_name.token();
                quote! {
                    Option::<#option_token>::from_bytes(&reply.body.data[read_bytes as usize..reply.body.data.len()], &mut read_bytes)
                }
            }
            IceType::CustomType(_) => {
                if self.return_proxy {
                    quote! {
                        let proxy_data = ProxyData::from_bytes(&reply.body.data[read_bytes as usize..reply.body.data.len()], &mut read_bytes)?;
                        let proxy_string = format!("{}:{} -h {} -p {}", proxy_data.id, if proxy_data.secure { "ssl" } else { "tcp" }, self.proxy.host, self.proxy.port);
                        let comm = ice_rs::communicator::Communicator::new();
                        let proxy = comm.string_to_proxy(&proxy_string)?;
                        #return_token::checked_cast(proxy)
                    }
                } else {
                    quote! {
                        #return_token::from_bytes(&reply.body.data[read_bytes as usize..reply.body.data.len()], &mut read_bytes)
                    }
                }
            }
            _ => {                
                quote! {
                    #return_token::from_bytes(&reply.body.data[read_bytes as usize..reply.body.data.len()], &mut read_bytes)
                }
            }
        }
    }
}
