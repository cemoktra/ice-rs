use crate::slice::types::IceType;
use __private::TokenStream;
use quote::*;


#[derive(Clone, Debug)]
pub struct FunctionArgument {
    pub id: TokenStream,
    pub r#type: IceType,
    pub out: bool
}

impl FunctionArgument {
    pub fn new(id: TokenStream, r#type: IceType, out: bool) -> FunctionArgument {
        FunctionArgument {
            id: id,
            r#type: r#type,
            out: out
        }
    }

    pub fn token(&self) -> TokenStream {
        let id = &self.id;
        let out = if self.out { 
            Some(quote! { &mut })
        } else {
            if self.r#type.as_ref() {
                Some(quote! { & })
            } else {
                None 
            }

        };
        let typename = self.r#type.token();
        quote! { #id: #out #typename }
    }

    pub fn serialize_output(&self) -> Option<TokenStream> {
        if self.out {
            let id_token = &self.id;
            let type_token = &self.r#type.token_from();
            Some(quote! {
                *#id_token = #type_token::from_bytes(&reply.body.data[read_bytes as usize..reply.body.data.len()], &mut read_bytes)?;
            })
        } else {
            None
        }
    }

    pub fn serialize_input(&self) -> Option<TokenStream> {
        let id_token = &self.id;
        if self.out {
            None
        } else {
            match &self.r#type {
                IceType::Optional(var_type, tag) => {
                    let option_type = var_type.token();
                    Some(quote! {
                        if let Some(value) = #id_token {
                            bytes.extend(OptionalFlag::new(#tag, #option_type::optional_type()).to_bytes()?);
                            bytes.extend(value.to_bytes()?);
                        }
                    })
                }
                _ => {
                    Some(quote! {
                        bytes.extend(#id_token.to_bytes()?);
                    })
                }
            }
        }
    }
}
