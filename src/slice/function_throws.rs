use quote::{__private::TokenStream, quote};

use super::types::IceType;

#[derive(Clone, Debug)]
pub struct FunctionThrows {
    pub r#type: Option<IceType>,
}

impl FunctionThrows {
    pub fn new(r#type: IceType) -> FunctionThrows {
        FunctionThrows {
            r#type: Some(r#type)
        }
    }

    pub fn empty() -> FunctionThrows {
        FunctionThrows {
            r#type: None
        }
    }

    pub fn token(&self) -> TokenStream {
        match &self.r#type {
            Some(throw) => {
                throw.token()                
            },
            _ => quote! {
                ProtocolError
            }
        }   
    }
}
