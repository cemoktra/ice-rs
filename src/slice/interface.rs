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

        let id_token = &self.id;
        let id_proxy_token = format_ident!("{}Prx", self.id.to_string());
        let type_id_token = format!("{}::{}", mod_path, self.ice_id);
        Ok(quote! {
            pub trait #id_token : IceObject {
                #decl_tokens
            }

            pub struct #id_proxy_token {
                proxy: Proxy
            }

            impl IceObject for #id_proxy_token {
                const TYPE_ID: &'static str = #type_id_token;
                fn dispatch<T: 'static + std::fmt::Debug + std::fmt::Display + FromBytes>(&mut self, op: &str, mode: u8, params: &Encapsulation) -> Result<ReplyData, Box<dyn std::error::Error>> {
                    let id = String::from(self.proxy.ident.clone());
                    let req = self.proxy.create_request(&id, op, mode, params);
                    self.proxy.make_request::<T>(&req)
                }
            }

            impl #id_token for #id_proxy_token {
                #impl_tokens
            }

            impl #id_proxy_token {
                pub fn unchecked_cast(proxy: Proxy) -> Result<Self, Box<dyn std::error::Error>> {
                    Ok(Self {
                        proxy: proxy,
                    })
                }

                pub fn checked_cast(proxy: Proxy) -> Result<Self, Box<dyn std::error::Error>> {
                    let mut my_proxy = Self::unchecked_cast(proxy)?;
            
                    if !my_proxy.ice_is_a()? {
                        return Err(Box::new(ProtocolError::new("ice_is_a() failed")));
                    }
                    Ok(my_proxy)
                }
            }
        })
    }
}
