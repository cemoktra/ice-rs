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

        Ok(quote! {
            #[derive(Debug, Clone, PartialEq, IceDerive)]
            pub struct #id_token {
                #(#member_tokens),*
            }
        })
    }
}