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
        arg_tokens.push(quote! {
            context: Option<HashMap<String, String>>
        });

        Ok(quote! {
            async fn #id_token (#(#arg_tokens),*) -> Result<#return_token, Box<dyn std::error::Error + Send + Sync>>;
        })
    }

    pub fn generate_server_decl(&self) -> Result<TokenStream, Box<dyn std::error::Error>> {       
        let id_token = &self.id;
        let return_token = self.return_type.token();
        let mut arg_tokens = vec![
            quote! { &mut self }
        ];
        arg_tokens.extend(self.arguments.iter().map(|arg| arg.token()).collect::<Vec<_>>());
        arg_tokens.push(quote! {
            context: Option<HashMap<String, String>>
        });

        Ok(quote! {
            async fn #id_token (#(#arg_tokens),*) -> #return_token;
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
        arg_tokens.push(quote! {
            context: Option<HashMap<String, String>>
        });
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
            async fn #id_token (#(#arg_tokens),*) -> Result<#return_token, Box<dyn std::error::Error + Send + Sync>> {
                #bytes_token
                #(#arg_serialize_input_tokens)*
                #reply_token self.proxy.dispatch::<#throw_token>(&String::from(#ice_id_token), #mode, &Encapsulation::from(bytes), context).await?;
                #read_token
                #(#arg_serialize_output_tokens)*
                #returned_token
            }
        })
    }

    fn wrap_result(&self) -> TokenStream {
        match self.return_type.r#type {
            IceType::Optional(_, tag) => {
                quote! { let wrapped_result = OptionalWrapper::new(#tag, result); }
            },
            _ => quote! { let wrapped_result = result; }
        }
    }

    pub fn generate_server_handler(&self) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let ice_id_token = self.ice_id.clone();
        let id_token = &self.id;
        let wrapped_result = self.wrap_result();
        
        let func_call = if self.arguments.len() > 0 {
            let mut arg_tokens = vec![];
            arg_tokens.extend(self.arguments.iter().map(|arg| arg.pass_argument()).collect::<Vec<_>>());
            // TODO: split non optionals and options as non optionals come first and optionals need special handling
            let decoded_tokens = self.arguments.iter().map(|arg| arg.decode_request()).collect::<Vec<_>>();
            let decoded_opt_tokens = self.arguments.iter().map(|arg| arg.decode_optional_request()).collect::<Vec<_>>();
            let encoded_outputs = self.arguments.iter().map(|arg| arg.encode_output()).collect::<Vec<_>>();
            let has_outputs = self.arguments.iter().any(|arg| arg.out);
            let mut_token = if has_outputs {
                quote!{ mut }
            } else {
                quote!{ }
            }; 

            quote!{
                let mut read_bytes = 0;
                #(#decoded_tokens)*
                #(#decoded_opt_tokens)*
                let result = self.server_impl.#id_token (#(#arg_tokens),*, None).await;
                #wrapped_result
                let #mut_token result = wrapped_result.to_bytes()?;
                #(#encoded_outputs)*                
            }
        } else {
            quote!{
                let result = self.server_impl.#id_token (None).await;
                #wrapped_result
                let result = wrapped_result.to_bytes()?;
            }
        };

        Ok(quote! {
            #ice_id_token => {
                #func_call
                Ok(ReplyData {
                    request_id: request.request_id,
                    status: 0,
                    body: Encapsulation::from(result)
                })
            },
        })
    }
}