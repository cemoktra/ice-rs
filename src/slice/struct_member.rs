use crate::slice::types::IceType;
use quote::{__private::TokenStream, quote};


#[derive(Clone, Debug)]
pub struct StructMember {
    pub id: TokenStream,
    pub ice_id: String,
    pub r#type: IceType
}

impl StructMember {
    pub fn empty() -> StructMember {
        StructMember {
            id: TokenStream::new(),
            ice_id: String::new(),
            r#type: IceType::VoidType
        }
    }

    pub fn declare(&self) -> TokenStream {
        let id_token = &self.id;
        let var_token = self.r#type.token();
        quote! {
            #id_token: #var_token
        }
    }

    pub fn to_bytes(&self) -> TokenStream {
        let id_token = &self.id;
        quote! {
            bytes.extend(self.#id_token.to_bytes()?);
        }
    }

    pub fn from_bytes(&self) -> TokenStream {
        let id_token = &self.id;
        let var_token = self.r#type.token();
        quote! {
            #id_token:  #var_token::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?
        }
    }
}