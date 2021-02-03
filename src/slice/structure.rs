use quote::{__private::TokenStream, quote};
use super::struct_member::StructMember;


#[derive(Clone, Debug)]
pub struct Struct {
    pub id: TokenStream,
    pub ice_id: String,
    pub members: Vec<StructMember>
}

impl Struct {
    pub fn empty() -> Struct {
        Struct {
            id: TokenStream::new(),
            ice_id: String::new(),
            members: Vec::new()
        }
    }

    pub fn add_member(&mut self, member: StructMember) {
        self.members.push(member);
    }

    pub fn generate(&self) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let id_token = &self.id;
        let member_tokens = self.members.iter().map(|member| {
            member.declare()
        }).collect::<Vec<_>>();
        let member_to_bytes_tokens = self.members.iter().map(|member| {
            member.to_bytes()
        }).collect::<Vec<_>>();
        let member_from_bytes_tokens = self.members.iter().map(|member| {
            member.from_bytes()
        }).collect::<Vec<_>>();

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
                    let obj = Self {
                        #(#member_from_bytes_tokens),*
                    };
                    *read_bytes = *read_bytes + read;
                    Ok(obj)
                }
            }
        })
    }
}