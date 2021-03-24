use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::Fields::Named;
use syn::Data::Struct;
use proc_macro2::TokenStream as TokenStream2;


#[proc_macro_derive(IceDerive)]
pub fn ice_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let blocks = vec![impl_ice_encode(&ast), impl_ice_decode(&ast)];
    let gen = quote! {
        #(#blocks)*
    };
    gen.into()
}

#[proc_macro_derive(IceEncode)]
pub fn ice_encode(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_ice_encode(&ast).into()
}

#[proc_macro_derive(IceDecode)]
pub fn ice_decode(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_ice_decode(&ast).into()
}


fn impl_ice_encode(ast: &syn::DeriveInput) -> TokenStream2 {
    let ident = &ast.ident;
    let mut members = Vec::new();

    match &ast.data {
        Struct(data) => {
            match &data.fields {
                Named(fields) => {
                    for field in fields.named.iter() {
                        let field_ident = field.ident.as_ref().unwrap();

                        members.push(quote! {
                            buffer.extend(self.#field_ident.to_bytes()?);
                        });
                    }
                },
                _ => {
                    panic!("IceDerive supports named fields only")
                }
            }
        }
        _ => {
            panic!("IceDerive supports structs only")
        }
    }

    quote! {
        impl ToBytes for #ident {
            fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error + Sync + Send>>
            {
                let mut buffer: Vec<u8> = Vec::new();
                #(#members)*
                Ok(buffer)
            }
        }
    }
}

fn impl_ice_decode(ast: &syn::DeriveInput) -> TokenStream2 {
    let ident = &ast.ident;
    let mut members = Vec::new();

    match &ast.data {
        Struct(data) => {
            match &data.fields {
                Named(fields) => {
                    for field in fields.named.iter() {
                        let field_ident = field.ident.as_ref().unwrap();
                        let field_type = &field.ty;
                        
                        members.push(quote! {
                            #field_ident: <#field_type>::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?
                        });
                    }
                },
                _ => {
                    panic!("IceDerive supports named fields only")
                }
            }
        }
        _ => {
            panic!("IceDerive supports structs only")
        }
    }

    quote! {
        impl FromBytes for #ident {
            fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
                let mut read = 0;
                let result = #ident {
                    #(#members),*
                };
                *read_bytes = *read_bytes + read;
                Ok(result)
            }
        }
    }
}