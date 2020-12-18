use crate::errors::*;
use crate::slice::enumeration::Enum;
use crate::slice::struct_decl::Struct;
use crate::slice::interface::Interface;
use crate::slice::exception::Exception;
use crate::slice::writer;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::collections::BTreeSet;
use inflector::cases::snakecase;

use super::types::IceType;


pub struct Module {
    pub name: String,
    pub full_name: String,
    sub_modules: Vec<Module>,
    enumerations: Vec<Enum>,
    exceptions: Vec<Exception>,
    structs: Vec<Struct>,
    interfaces: Vec<Interface>,
    typedefs: Vec<(String, IceType)>,
}

impl Module {
    pub fn new() -> Module {
        Module {
            name: String::from(""),
            full_name: String::from(""),
            sub_modules: vec![],
            enumerations: vec![],
            structs: vec![],
            interfaces: vec![],
            exceptions: vec![],
            typedefs: vec![]
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

    pub fn add_module(&mut self, module: Module) {
        self.sub_modules.push(module);
    }

    pub fn add_sub_module(&mut self, name: &str) -> Result<&mut Module, Box<dyn std::error::Error>> {
        if name.len() == 0 {
            return Err(Box::new(ParsingError {}));
        }
        self.sub_modules.push(Module{
            name: String::from(name),
            full_name: format!("{}::{}", self.full_name, name),
            sub_modules: vec![],
            enumerations: vec![],
            structs: vec![],
            interfaces: vec![],
            exceptions: vec![],
            typedefs: vec![]
        });
        self.sub_modules.last_mut().ok_or(Box::new(ParsingError {}))
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

    pub fn add_exception(&mut self, exception: Exception) {
        self.exceptions.push(exception);
    }

    pub fn add_typedef(&mut self, id: &str, vartype: IceType) {
        self.typedefs.push((String::from(id), vartype.clone()));
    }
    
    pub fn generate(&self, dest: &Path, context: &str) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::create_dir_all(dest)?;
        let mod_file = &dest.join(Path::new("mod.rs"));
        let mut file = File::create(mod_file)?;

        file.write_all("// This file has been generated.\n\n".as_bytes())?;

        // build up use statements
        let mut uses: BTreeSet<String> = BTreeSet::new();
        
        if self.has_dict() {
            uses.insert(String::from("use std::collections::HashMap;\n"));
        }

        if self.enumerations.len() > 0 || self.structs.len() > 0 || self.interfaces.len() > 0 {
            uses.insert(String::from("use ice_rs::errors::*;\n"));
        }
        if self.enumerations.len() > 0 {
            uses.insert(String::from("use num_enum::TryFromPrimitive;\n"));
            uses.insert(String::from("use std::convert::TryFrom;\n"));
            uses.insert(String::from("use ice_rs::encoding::IceSize;\n"));
            uses.insert(String::from("use ice_rs::encoding::{\n   ToBytes, FromBytes\n};\n"));
        }
        // TODO: use statements from structs from different modules
        if self.structs.len() > 0 {
            uses.insert(String::from("use ice_rs::encoding::{\n   ToBytes, FromBytes\n};\n"));
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

        for (id, vartype) in &self.typedefs {
            writer::write(&mut file, &format!("type {} = {};\n", id, vartype.rust_type()), 0)?;
        }
        writer::write(&mut file, "\n", 0)?;
       
        for enumeration in &self.enumerations {
            enumeration.generate(&mut file)?;
        }

        file.write_all("\n".as_bytes())?;

        for struct_decl in &self.structs {
            struct_decl.generate(&mut file)?;
        }

        for exception in &self.exceptions {
            exception.generate(&mut file)?;
        }

        file.write_all("\n".as_bytes())?;

        for interface in &self.interfaces {
            interface.generate(&mut file, &self.full_name, context)?;
        }

        Ok(())
    }
}