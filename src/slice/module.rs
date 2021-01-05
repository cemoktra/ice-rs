use crate::errors::*;
use crate::slice::enumeration::Enum;
use crate::slice::structure::Struct;
use crate::slice::interface::Interface;
use crate::slice::exception::Exception;
use crate::slice::class::Class;
use crate::slice::writer::Writer;
use std::path::Path;
use std::fs::File;
use std::collections::BTreeSet;
use inflector::cases::snakecase;

use super::types::IceType;


struct UseStatements {
    uses: BTreeSet<String>,
}

impl UseStatements {
    fn new() -> UseStatements {
        UseStatements {
            uses: BTreeSet::new()
        }
    }

    fn use_crate(&mut self, crate_name: &str) {
        self.uses.insert(String::from(crate_name));
    }

    fn generate(&self, writer: &mut Writer) -> Result<(), Box<dyn std::error::Error>>{
        for crate_name in &self.uses {
            writer.generate_use(crate_name, 0)?;
        }
        Ok(())
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
    classes: Vec<Class>
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
            typedefs: vec![],
            classes: vec![]
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
            typedefs: vec![],
            classes: vec![]
        });
        self.sub_modules.last_mut().ok_or(Box::new(ParsingError {}))
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

    fn uses(&self) -> UseStatements {
        let mut use_statements = UseStatements::new();
        
        if self.has_dict() {
            use_statements.use_crate("std::collections::HashMap");
        }

        if self.enumerations.len() > 0 || self.structs.len() > 0 || self.interfaces.len() > 0 {
            use_statements.use_crate("ice_rs::errors::*");
        }
        
        if self.enumerations.len() > 0 {
            use_statements.use_crate("num_enum::TryFromPrimitive");
            use_statements.use_crate("std::convert::TryFrom");
            use_statements.use_crate("ice_rs::encoding::*");
        }
        // TODO: use statements from structs from different modules
        if self.structs.len() > 0 {
            use_statements.use_crate("ice_rs::encoding::*");
        }

        if self.interfaces.len() > 0 {
            use_statements.use_crate("ice_rs::encoding::*");
            use_statements.use_crate("ice_rs::proxy::Proxy");
            use_statements.use_crate("ice_rs::iceobject::IceObject");
            use_statements.use_crate("ice_rs::protocol::*");
        }

        use_statements
    }

    pub fn generate(&self, dest: &Path) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::create_dir_all(dest)?;
        let mod_file = &dest.join(Path::new("mod.rs"));

        let mut writer = Writer::new(File::create(mod_file)?);
        writer.write("// This file has been generated.", 0)?;
        writer.blank_line()?;

        // build up use statements
        self.uses().generate(&mut writer)?;

        for sub_module in &self.sub_modules {
            let mod_name = sub_module.snake_name();
            writer.generate_mod(&mod_name, 0)?;
            sub_module.generate(&dest.join(Path::new(&mod_name)))?;
        }
        writer.blank_line()?;

        for (id, vartype) in &self.typedefs {
            writer.generate_typedef(id, &vartype.rust_type(), 0)?;
        }
        writer.blank_line()?;
       
        for enumeration in &self.enumerations {
            enumeration.generate(&mut writer)?;
        }
        writer.blank_line()?;

        for structure in &self.structs {
            structure.generate(&mut writer)?;
        }
        writer.blank_line()?;

        for class in &self.classes {
            class.generate(&mut writer)?;
        }
        writer.blank_line()?;

        for exception in &self.exceptions {
            exception.generate(&mut writer)?;
        }
        writer.blank_line()?;

        for interface in &self.interfaces {
            interface.generate(&mut writer, &self.full_name)?;
        }

        Ok(())
    }
}