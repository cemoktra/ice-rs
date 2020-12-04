use crate::errors::Error;
use crate::slice::enumeration::Enum;
use crate::slice::struct_decl::Struct;
use crate::slice::interface::Interface;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use inflector::cases::snakecase;


pub struct Module {
    pub name: String,
    pub full_name: String,
    sub_modules: Vec<Module>,
    enumerations: Vec<Enum>,
    structs: Vec<Struct>,
    interfaces: Vec<Interface>,
}

impl Module {
    pub fn root() -> Module {
        Module {
            name: String::from(""),
            full_name: String::from(""),
            sub_modules: vec![],
            enumerations: vec![],
            structs: vec![],
            interfaces: vec![]
        }
    }

    pub fn snake_name(&self) -> String {
        snakecase::to_snake_case(&self.name)
    }

    pub fn add_sub_module(&mut self, name: &str) -> Result<&mut Module, Error> {
        if name.len() == 0 {
            return Err(Error::Unexpected);
        }
        self.sub_modules.push(Module{
            name: String::from(name),
            full_name: format!("{}::{}", self.full_name, name),
            sub_modules: vec![],
            enumerations: vec![],
            structs: vec![],
            interfaces: vec![]
        });
        self.sub_modules.last_mut().ok_or(Error::Unexpected)
    }

    pub fn add_enum(&mut self, enumeration: &Enum) {
        self.enumerations.push(enumeration.clone());
    }

    pub fn add_struct(&mut self, struct_decl: &Struct) {
        self.structs.push(struct_decl.clone());
    }

    pub fn add_interface(&mut self, interface: &Interface) {
        self.interfaces.push(interface.clone());
    }

    pub fn write(&self, dest: &Path, context: &str) -> Result<(), Error> {
        std::fs::create_dir_all(dest)?;
        let mod_file = &dest.join(Path::new("mod.rs"));
        let mut file = File::create(mod_file)?;

        file.write_all("// This file has been generated.\n\n".as_bytes())?;

        // build up use statements
        let mut uses: HashSet<String> = HashSet::new();
        if self.enumerations.len() > 0 || self.structs.len() > 0 || self.interfaces.len() > 0 {
            uses.insert(String::from("use ice_rs::errors::Error;"));
        }
        if self.enumerations.len() > 0 {
            uses.insert(String::from("use num_enum::TryFromPrimitive;"));
            uses.insert(String::from("use std::convert::TryFrom;"));
            uses.insert(String::from("use ice_rs::encoding::IceSize;"));
            uses.insert(String::from("use ice_rs::encoding::{\n   ToBytes, FromBytes, AsEncapsulation, FromEncapsulation\n};"));
        }
        // TODO: use statements from structs from different modules
        if self.structs.len() > 0 {
            uses.insert(String::from("use ice_rs::encoding::{\n   ToBytes, FromBytes, AsEncapsulation, FromEncapsulation\n};"));
        }

        if self.interfaces.len() > 0 {
            uses.insert(String::from("use ice_rs::proxy::Proxy;"));
            uses.insert(String::from("use ice_rs::iceobject::IceObject;"));
            uses.insert(String::from("use ice_rs::protocol::{Encapsulation, ReplyData};"));
        }

        // write use statements
        for use_statement in &uses {
            file.write_all(format!("{}\n", use_statement).as_bytes())?;
        }


        for sub_module in &self.sub_modules {
            let mod_name = sub_module.snake_name();
            let mod_use = "pub mod ".to_owned() + &mod_name + ";\n";
            file.write_all(mod_use.as_bytes())?;
            sub_module.write(&dest.join(Path::new(&mod_name)), context)?;
        }
        file.write_all("\n".as_bytes())?;


        

        for enumeration in &self.enumerations {
            enumeration.write(&mut file)?;
        }

        file.write_all("\n".as_bytes())?;

        for struct_decl in &self.structs {
            struct_decl.write(&mut file)?;
        }

        file.write_all("\n".as_bytes())?;

        for interface in &self.interfaces {
            interface.write(&mut file, &self.full_name, context)?;
        }

        Ok(())
    }
}