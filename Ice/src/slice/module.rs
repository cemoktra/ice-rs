use crate::errors::Error;
use crate::slice::enumeration::Enum;
use crate::slice::struct_decl::Struct;
use crate::slice::interface::Interface;
use crate::slice::writer;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::collections::BTreeSet;
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
    pub fn new() -> Module {
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

    pub fn add_module(&mut self, module: Module) {
        self.sub_modules.push(module);
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

    pub fn add_enum(&mut self, enumeration: Enum) {
        self.enumerations.push(enumeration);
    }

    pub fn add_struct(&mut self, struct_decl: Struct) {
        self.structs.push(struct_decl);
    }

    pub fn add_interface(&mut self, interface: Interface) {
        self.interfaces.push(interface);
    }

    pub fn generate(&self, dest: &Path, context: &str) -> Result<(), Error> {
        std::fs::create_dir_all(dest)?;
        let mod_file = &dest.join(Path::new("mod.rs"));
        let mut file = File::create(mod_file)?;

        file.write_all("// This file has been generated.\n\n".as_bytes())?;

        // build up use statements
        let mut uses: BTreeSet<String> = BTreeSet::new();
        if self.enumerations.len() > 0 || self.structs.len() > 0 || self.interfaces.len() > 0 {
            uses.insert(String::from("use ice_rs::errors::Error;\n"));
        }
        if self.enumerations.len() > 0 {
            uses.insert(String::from("use num_enum::TryFromPrimitive;\n"));
            uses.insert(String::from("use std::convert::TryFrom;\n"));
            uses.insert(String::from("use ice_rs::encoding::IceSize;\n"));
            uses.insert(String::from("use ice_rs::encoding::{\n   ToBytes, FromBytes, AsEncapsulation, FromEncapsulation\n};\n"));
        }
        // TODO: use statements from structs from different modules
        if self.structs.len() > 0 {
            uses.insert(String::from("use ice_rs::encoding::{\n   ToBytes, FromBytes, AsEncapsulation, FromEncapsulation\n};\n"));
        }

        if self.interfaces.len() > 0 {
            uses.insert(String::from("use ice_rs::proxy::Proxy;\n"));
            uses.insert(String::from("use ice_rs::iceobject::IceObject;\n"));
            uses.insert(String::from("use ice_rs::protocol::{Encapsulation, ReplyData};\n"));
        }

        // write use statements
        for use_statement in &uses {
            writer::write(&mut file, use_statement, 0)?;
        }


        for sub_module in &self.sub_modules {
            let mod_name = sub_module.snake_name();
            writer::write(&mut file, &("pub mod ".to_owned() + &mod_name + ";\n"), 0)?;
            sub_module.generate(&dest.join(Path::new(&mod_name)), context)?;
        }
        writer::write(&mut file, "\n", 0)?;
       
        for enumeration in &self.enumerations {
            enumeration.generate(&mut file)?;
        }

        file.write_all("\n".as_bytes())?;

        for struct_decl in &self.structs {
            struct_decl.generate(&mut file)?;
        }

        file.write_all("\n".as_bytes())?;

        for interface in &self.interfaces {
            interface.generate(&mut file, &self.full_name, context)?;
        }

        Ok(())
    }
}