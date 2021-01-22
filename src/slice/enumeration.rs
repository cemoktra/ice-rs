use quote::{__private::TokenStream, format_ident, quote};

#[derive(Clone, Debug)]
pub struct Enum {
    pub id: TokenStream,
    pub ice_id: String,
    variants: Vec<TokenStream>,
    next_value: i32
}

impl Enum {
    pub fn empty() -> Enum {
        Enum {
            id: TokenStream::new(),
            ice_id: String::new(),
            variants: vec![],
            next_value: 0
        }
    }

    pub fn add_variant(&mut self, name: &str, value: Option<i32>) {
        let value = match value {
            Some(value) => {
                self.next_value = value + 1;
                value
            },
            None => {
                let value = self.next_value;
                self.next_value = value + 1;
                value
            }
        };
        let id = format_ident!("{}", name);
        self.variants.push(quote! {
            #id = #value
        });
    }

    pub fn generate(&self) -> Result<TokenStream, Box<dyn std::error::Error>> {
        let id_token = &self.id;
        let variant_tokens = self.variants.iter().map(|variant| {
            quote! {
                #variant
            }
        }).collect::<Vec<_>>();

        Ok(quote! {
            #[derive(Debug, Copy, Clone, TryFromPrimitive, PartialEq)]
            #[repr(i32)]
            pub enum #id_token {
                #(#variant_tokens),*
            }

            impl OptionalType for #id_token {
                fn optional_type() -> u8 {
                    4
                }
            }

            impl ToBytes for #id_token {
                fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
                    let mut bytes = Vec::new();
                    bytes.extend(IceSize{size: *self as i32}.to_bytes()?);
                    Ok(bytes)
                }
            }

            impl FromBytes for #id_token {
                fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Box<dyn std::error::Error>>
                where Self: Sized {
                    let mut read = 0;
                    let enum_value =  IceSize::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?.size;
                    *read_bytes = *read_bytes + read;
                    match #id_token::try_from(enum_value) {
                        Ok(enum_type) => Ok(enum_type),
                        _ => Err(Box::new(ProtocolError::new("Cannot convert int to enum")))
                    }
                }
            }
        })
    }
}