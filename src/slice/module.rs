use crate::slice::enumeration::Enum;
use crate::slice::structure::Struct;
use crate::slice::interface::Interface;
use crate::slice::exception::Exception;
use crate::slice::class::Class;
use crate::slice::writer::Writer;
use std::path::Path;
use std::fs::File;
use std::collections::{BTreeSet, BTreeMap};
use std::rc::Rc;
use std::cell::RefCell;
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
        if self.structs.len() > 0 {
            use_statements.use_crate("ice_rs::encoding::*");

            for item in &self.structs {
                for (_, var_type) in &item.members {
                    match var_type {
                        IceType::CustomType(name) => {
                            let use_statement = self.type_map.as_ref().borrow().get(name).unwrap().clone();
                            if !use_statement.eq(&self.snake_name()) {
                                use_statements.use_crate(&format!("crate::{}::{}::{}", super_mod, use_statement, name));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        if self.classes.len() > 0 {
            use_statements.use_crate("ice_rs::encoding::*");

            for item in &self.classes {
                for (_, var_type) in &item.members {
                    match var_type {
                        IceType::CustomType(name) => {
                            let use_statement = self.type_map.as_ref().borrow().get(name).unwrap().clone();
                            if !use_statement.eq(&self.snake_name()) {
                                use_statements.use_crate(&format!("crate::{}::{}::{}", super_mod, use_statement, name));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        if self.interfaces.len() > 0 {
            use_statements.use_crate("ice_rs::encoding::*");
            use_statements.use_crate("ice_rs::proxy::Proxy");
            use_statements.use_crate("ice_rs::iceobject::IceObject");
            use_statements.use_crate("ice_rs::protocol::*");

            for item in &self.interfaces {
                for func in &item.functions {
                    for (_, var_type, _) in &func.arguments {
                        match var_type {
                            IceType::CustomType(name) => {
                                let use_statement = self.type_map.as_ref().borrow().get(name).unwrap().clone();
                                if !use_statement.eq(&self.snake_name()) {
                                    use_statements.use_crate(&format!("crate::{}::{}::{}", super_mod, use_statement, name));
                                }
                            }
                            _ => {}
                        }
                    }

                    match &func.throws {
                        Some(throws) => {
                            match throws {
                                IceType::CustomType(name) => {
                                    let use_statement = self.type_map.as_ref().borrow().get(name).unwrap().clone();
                                    if !use_statement.eq(&self.snake_name()) {
                                        use_statements.use_crate(&format!("crate::{}::{}::{}", super_mod, use_statement, name));
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        use_statements
    }

    pub fn generate(&self, dest: &Path, mod_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::create_dir_all(dest)?;
        let mod_file = &dest.join(Path::new("mod.rs"));
        
        let mut writer = Writer::new(File::create(mod_file)?);
        writer.write("// This file has been generated.", 0)?;
        writer.blank_line()?;

        // build up use statements
        let mut use_path = vec![];
        if mod_path.len() > 0 {
            use_path.push(mod_path);
        }
        use_path.push(dest.iter().last().unwrap().to_str().unwrap());

        let new_mod_path = use_path.join("::");
        self.uses(&new_mod_path).generate(&mut writer)?;

        for sub_module in &self.sub_modules {
            let mod_name = sub_module.snake_name();
            writer.generate_mod(&mod_name, 0)?;
            sub_module.generate(&dest.join(Path::new(&mod_name)), &new_mod_path)?;
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