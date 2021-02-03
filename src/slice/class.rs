use crate::slice::types::IceType;
use quote::{__private::TokenStream, quote};

use super::struct_member::StructMember;


#[derive(Clone, Debug)]
pub struct Class {
    pub id: TokenStream,
    pub ice_id: String,
    pub members: Vec<StructMember>,
    pub extends: Option<IceType>,
}

impl Class {
    pub fn empty() -> Class {
        Class {
            id: TokenStream::new(),
            ice_id: String::new(),
            members: Vec::new(),
            extends: None
        }
    }

    pub fn add_member(&mut self, member: StructMember) {
        self.members.push(member);
    }

    pub fn generate(&self) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let id_token = &self.id;
        let mut member_tokens = self.members.iter().map(|member| {
            member.declare()
        }).collect::<Vec<_>>();
        let member_to_bytes_tokens = self.members.iter().map(|member| {
            member.to_bytes()
        }).collect::<Vec<_>>();
        let member_from_bytes_tokens = self.members.iter().map(|member| {
            let id_token = &member.id;
            let var_token = &member.r#type.token_from();
            match member.r#type {
                IceType::Optional(_, _) => {
                    quote! {
                        let mut #id_token = None
                    }
                }
                _ => {
                    quote! {
                        let #id_token = #var_token::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?
                    }
                }
            }
        }).collect::<Vec<_>>();

        let mut member_to_struct = self.members.iter().map(|member| {
            let id_token = &member.id;
            quote! {
                #id_token: #id_token
            }
        }).collect::<Vec<_>>();

        if self.extends.is_some() { 
            let token = self.extends.as_ref().unwrap().token();
            member_tokens.push(quote!{
                extends: #token
            });
            member_to_struct.push(quote!{
                extends: #token::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?
            });
        }

        let has_optionals = self.members.iter().any(|member| {
            match member.r#type {
                IceType::Optional(_, _) => { true },
                _ => false
            }
        });

        let optional_tokens = self.members.iter()
        .filter_map(|member| {
            let id_token = &member.id;            
            match &member.r#type {
                IceType::Optional(option_type, tag) => {
                    let var_token = option_type.token();
                    Some(quote! {
                        #tag => {
                            #id_token = Some(#var_token::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?);
                        }
                    })
                },
                _ => None
            }
        }).collect::<Vec<_>>();

        let optional_from = if has_optionals {
            Some(quote! {
                while read < bytes.len() as i32 {
                    let flag_byte = bytes[read as usize..bytes.len()].first().unwrap();
                    if *flag_byte == 0xFF {
                        break;
                    }
                    let flag = OptionalFlag::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
                    match flag.tag {
                        #(#optional_tokens),*
                        _ => {
                            if flags.last_slice {
                                return Err(Box::new(ProtocolError::new("Last slice not expected")));
                            } else {
                                read = read - 1;
                                break;
                            }
                        }
                    }
                }
            })
        } else {
            None
        };

        // TODO: ToBytes incomplete
        Ok(quote! {
            #[derive(Debug, Clone, PartialEq)]
            pub struct #id_token {
                #(#member_tokens),*
            }

            impl ToBytes for #id_token {
                fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
                    let mut bytes = Vec::new();
                    #(#member_to_bytes_tokens);*;
                    Ok(bytes)
                }
            }

            impl FromBytes for #id_token {
                fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Box<dyn std::error::Error + Send + Sync>>
                where Self: Sized {
                    let mut read = 0;
                    let marker = u8::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
                    if marker != 1 && marker != 255 {
                        read = 0;
                    }
                    let flags = SliceFlags::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
                    match flags.type_id {
                        SliceFlagsTypeEncoding::StringTypeId => {
                            let _slice_name = String::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;
                        }
                        SliceFlagsTypeEncoding::CompactTypeId => {
                            todo!()
                        }
                        SliceFlagsTypeEncoding::IndexTypeId => {
                            todo!()
                        }
                        SliceFlagsTypeEncoding::NoTypeId => {}
                    }

                    #(#member_from_bytes_tokens);*;
                    #optional_from

                    let obj = Self{
                        #(#member_to_struct),*
                    };
                    *read_bytes = *read_bytes + read;
                    Ok(obj)
                }
            }
        })
    }
}