use crate::{errors::ParsingError, slice::enumeration::Enum};
use crate::slice::structure::Struct;
use crate::slice::interface::Interface;
use crate::slice::exception::Exception;
use crate::slice::class::Class;
use std::{path::Path, process::Stdio};
use std::fs::File;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::cell::RefCell;
use inflector::cases::{pascalcase, snakecase};
use quote::{__private::TokenStream, format_ident, quote};
use std::io::Write;
use super::types::IceType;
use std::process::Command;


struct UseStatements {
    uses: BTreeMap<String, TokenStream>,
}

impl UseStatements {
    fn new() -> UseStatements {
        UseStatements {
            uses: BTreeMap::new()
        }
    }

    fn use_crate(&mut self, token: TokenStream) {
        self.uses.insert(token.to_string(), token);
    }

    fn generate(&self) -> Result<TokenStream, Box<dyn std::error::Error>>{
        let tokens = self.uses.iter().map(|(_, token)| {
            quote! {
                #token;
            }
        }).collect::<Vec<_>>();
        Ok(quote! {
            #(#tokens)*
        })
    }
}

pub struct Module {
    pub name: String,
    pub full_name: String,
    pub sub_modules: Vec<Module>,
    enumerations: Vec<Enum>,
    exceptions: Vec<Exception>,
    structs: Vec<Struct>,
    interfaces: Vec<Interface>,
    typedefs: Vec<(String, IceType)>,
    classes: Vec<Class>,
    pub type_map: Rc<RefCell<BTreeMap<String, String>>>
}

impl Module {
    pub fn new(type_map: Rc<RefCell<BTreeMap<String, String>>>) -> Module {
        Module {
            name: String::new(),
            full_name: String::new(),
            sub_modules: vec![],
            enumerations: vec![],
            structs: vec![],
            interfaces: vec![],
            exceptions: vec![],
            typedefs: vec![],
            classes: vec![],
            type_map: type_map
        }
    }

    pub fn has_dict(&self) -> bool {
        for (_, var) in &self.typedefs {
            match var {
                IceType::DictType(_, _) => return true,
                _ => {}
            }
        }
        false
    }

    pub fn snake_name(&self) -> String {
        snakecase::to_snake_case(&self.name)
    }

    pub fn add_enum(&mut self, enumeration: Enum) {
        self.enumerations.push(enumeration);
    }

    pub fn add_struct(&mut self, structure: Struct) {
        self.structs.push(structure);
    }

    pub fn add_interface(&mut self, interface: Interface) {
        self.interfaces.push(interface);
    }

    pub fn add_exception(&mut self, exception: Exception) {
        self.exceptions.push(exception);
    }

    pub fn add_typedef(&mut self, id: &str, vartype: IceType) {
        self.typedefs.push((String::from(id), vartype.clone()));
    }

    pub fn add_class(&mut self, class: Class) {
        self.classes.push(class);
    }

    fn uses(&self, super_mod: &str) -> UseStatements {
        let mut use_statements = UseStatements::new();

        if self.has_dict() {
            use_statements.use_crate(quote! { use std::collections::HashMap });
        }

        if self.enumerations.len() > 0 || self.structs.len() > 0 || self.interfaces.len() > 0 {
            use_statements.use_crate(quote! { use ice_rs::errors::* });
        }

        if self.enumerations.len() > 0 {
            use_statements.use_crate(quote! { use num_enum::TryFromPrimitive });
            use_statements.use_crate(quote! { use std::convert::TryFrom });
            use_statements.use_crate(quote! { use ice_rs::encoding::* });
        }

        if self.structs.len() > 0 {
            use_statements.use_crate(quote! { use ice_rs::encoding::* });

            for item in &self.structs {
                for member in &item.members {
                    match &member.r#type {
                        IceType::CustomType(name) => {
                            let use_statement = self.type_map.as_ref().borrow().get(name).unwrap().clone();
                            if !use_statement.eq(&self.snake_name()) {
                                let super_token = format_ident!("{}", super_mod);
                                let use_token = format_ident!("{}", use_statement);
                                use_statements.use_crate(quote! { use crate::#super_token::#use_token::* });
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        if self.classes.len() > 0 {
            use_statements.use_crate(quote! { use ice_rs::encoding::* });

            for item in &self.classes {
                for member in &item.members {
                    match &member.r#type {
                        IceType::CustomType(name) => {
                            let use_statement = self.type_map.as_ref().borrow().get(name).unwrap().clone();
                            if !use_statement.eq(&self.snake_name()) {
                                let super_token = format_ident!("{}", super_mod);
                                let use_token = format_ident!("{}", use_statement);
                                use_statements.use_crate(quote! { use crate::#super_token::#use_token::* });
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        if self.interfaces.len() > 0 {
            use_statements.use_crate(quote! { use ice_rs::encoding::* });
            use_statements.use_crate(quote! { use ice_rs::proxy::Proxy });
            use_statements.use_crate(quote! { use ice_rs::iceobject::IceObject });
            use_statements.use_crate(quote! { use ice_rs::protocol::* });

            for item in &self.interfaces {
                for func in &item.functions {
                    use_statements.use_crate(quote! { use std::collections::HashMap });

                    for arg in &func.arguments {
                        match &arg.r#type {
                            IceType::CustomType(name) => {
                                let use_statement = self.type_map.as_ref().borrow().get(name).unwrap().clone();
                                if !use_statement.eq(&self.snake_name()) {
                                    let super_token = format_ident!("{}", super_mod);
                                    let use_token = format_ident!("{}", use_statement);
                                    use_statements.use_crate(quote! { use crate::#super_token::#use_token::* });
                                }
                            }
                            _ => {}
                        };
                    }

                    match &func.throws.r#type {
                        Some(throws) => {
                            match throws {
                                IceType::CustomType(name) => {
                                    let use_statement = self.type_map.as_ref().borrow().get(name).unwrap().clone();
                                    if !use_statement.eq(&self.snake_name()) {
                                        let super_token = format_ident!("{}", super_mod);
                                        let use_token = format_ident!("{}", use_statement);
                                        use_statements.use_crate(quote! { use crate::#super_token::#use_token::* });
                                    }
                                }
                                _ => {}
                            };
                        }
                        _ => {}
                    };
                }
            }
        }

        use_statements
    }

    pub fn generate(&self, dest: &Path, mod_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut tokens = vec![];
        tokens.push(quote! {
            // This file has been generated.
            #[allow(dead_code)]
            #[allow(unused_imports)]
        });

        // build up use statements
        let mut use_path = mod_path;
        if use_path.len() == 0 {
            use_path = dest.iter().last().unwrap().to_str().unwrap();
        }
        tokens.push(self.uses(&use_path).generate()?);

        for sub_module in &self.sub_modules {
            let mod_name = sub_module.snake_name();
            let ident = format_ident!("{}", mod_name);
            tokens.push(quote! {
                pub mod #ident;
            });
            sub_module.generate(&dest.join(Path::new(&mod_name)), &use_path)?;
        }

        for (id, vartype) in &self.typedefs {
            let id_str = format_ident!("{}", pascalcase::to_pascal_case(&id));
            let var_token = vartype.token();
            tokens.push(quote! {
                type #id_str = #var_token;
            });
        }

        for enumeration in &self.enumerations {
            tokens.push(enumeration.generate()?);
        }

        for structure in &self.structs {
            tokens.push(structure.generate()?);
        }

        for class in &self.classes {
            tokens.push(class.generate()?);
        }

        for exception in &self.exceptions {
            tokens.push(exception.generate()?);
        }

        for interface in &self.interfaces {
            tokens.push(interface.generate(&self.full_name)?);
        }

        let mod_token = quote! { #(#tokens)* };

        std::fs::create_dir_all(dest)?;
        let mod_file = &dest.join(Path::new("mod.rs")); 
        let mut child = Command::new("rustfmt")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        {
            let stdin = child.stdin.as_mut().ok_or(ParsingError::new("Could not get stdin of rustfmt process"))?;
            stdin.write_all(mod_token.to_string().as_bytes())?;
        }    
        let output = child.wait_with_output()?;
        let mut file = File::create(mod_file)?;
        match file.write_all(&output.stdout) {
            Ok(_) => Ok(()),
            Err(_) =>  Err(Box::new(ParsingError::new("Could not write file")))
        }
    }
}