use crate::slice::types::IceType;
use quote::{__private::TokenStream, quote};

use super::{function_argument::FunctionArgument, function_return::FunctionReturn, function_throws::FunctionThrows};


#[derive(Clone, Debug)]
pub struct Function {
    pub id: TokenStream,
    pub ice_id: String,
    pub return_type: FunctionReturn,
    pub arguments: Vec<FunctionArgument>,
    pub throws: FunctionThrows,
    idempotent: bool
}

impl Function {
    pub fn empty() -> Function {
        Function {
            id: TokenStream::new(),
            ice_id: String::new(),
            return_type: FunctionReturn::empty(),
            arguments: Vec::new(),
            throws: FunctionThrows::empty(),
            idempotent: false,
        }
    }

    pub fn set_idempotent(&mut self) {
        self.idempotent = true;
    }

    pub fn add_argument(&mut self, arg: FunctionArgument) {
        self.arguments.push(arg);
    }

    pub fn generate_decl(&self) -> Result<TokenStream, Box<dyn std::error::Error>> {       
        let id_token = &self.id;
        let return_token = self.return_type.token();
        let mut arg_tokens = vec![
            quote! { &mut self }
        ];
        arg_tokens.extend(self.arguments.iter().map(|arg| arg.token()).collect::<Vec<_>>());

        Ok(quote! {
            fn #id_token (#(#arg_tokens),*) -> Result<#return_token, Box<dyn std::error::Error>>;
        })
    }

    // TODO: return token stream
    pub fn generate_impl(&self) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let id_token = &self.id;
        let return_token = self.return_type.token();
        let mut arg_tokens = vec![
            quote! { &mut self }
        ];
        arg_tokens.extend(self.arguments.iter().map(|arg| arg.token()).collect::<Vec<_>>());
        let arg_require_result = self.arguments.iter().any(|arg| arg.out);
        let arg_serialize_input_tokens = self.arguments.iter().map(|arg| arg.serialize_input()).collect::<Vec<_>>();
        let arg_serialize_output_tokens = self.arguments.iter().map(|arg| arg.serialize_output()).collect::<Vec<_>>();
        let require_result = arg_require_result || match self.return_type.r#type {
            IceType::VoidType => false,
            _ => true
        };
        let mut reply_token = None;
        let mut read_token = None;
        let throw_token = self.throws.token();
        if require_result {
            reply_token = Some(quote! {
                let reply =    
            });
            read_token = Some(quote! {
                let mut read_bytes: i32 = 0;  
            })
        }

        let ice_id_token = &self.ice_id;
        let mode = if self.idempotent { 1u8 } else { 0u8 };
        let returned_token = self.return_type.return_token();
        let bytes_token = if arg_serialize_input_tokens.len() > 0 {
            quote! { let mut bytes = Vec::new(); }
        } else {
            quote! { let bytes = Vec::new(); }
        };

        Ok(quote! {
            fn #id_token (#(#arg_tokens),*) -> Result<#return_token, Box<dyn std::error::Error>> {
                
                #(#arg_serialize_input_tokens)*
                #bytes_token
                #reply_token self.dispatch::<#throw_token>(&String::from(#ice_id_token), #mode, &Encapsulation::from(bytes))?;
                #read_token
                #(#arg_serialize_output_tokens)*
                #returned_token
            }
        })
    }
}