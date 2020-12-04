use crate::slice::types::IceType;
use crate::errors::Error;
use std::fs::File;
use std::io::prelude::*;
use inflector::cases::snakecase;


#[derive(Clone, Debug)]
pub struct Function {
    name: String,
    return_type: IceType,
    arguments: Vec<(String, IceType)>
}

impl Function {
    pub fn new(name: &str, return_type: IceType) -> Function {
        Function {
            name: String::from(name),
            return_type: return_type,
            arguments: Vec::new()
        }
    }

    pub fn function_name(&self) -> String {
        snakecase::to_snake_case(&self.name)
    }

    pub fn add_argument(&mut self, name: &str, var_type: IceType) {
        self.arguments.push((String::from(name), var_type));
    }

    pub fn write_decl(&self, file: &mut File) -> Result<(), Error> {
        file.write_all(format!("    fn {}(&mut self", self.function_name()).as_bytes())?;
        for (key, var_type) in &self.arguments {
            file.write_all(format!(", {}: &{}", snakecase::to_snake_case(key), var_type.rust_type()).as_bytes())?;
        }
        file.write_all(format!(") -> Result<{}, Error>;\n", self.return_type.rust_type()).as_bytes())?;
        Ok(())
    }

    pub fn write_impl(&self, file: &mut File) -> Result<(), Error> {
        file.write_all(format!("    fn {}(&mut self", self.function_name()).as_bytes())?;
        for (key, var_type) in &self.arguments {
            file.write_all(format!(", {}: &{}", snakecase::to_snake_case(key), var_type.rust_type()).as_bytes())?;
        }
        file.write_all(format!(") -> Result<{}, Error> {{ \n", self.return_type.rust_type()).as_bytes())?;

        match self.return_type {
            IceType::VoidType => file.write_all(format!("        self.dispatch(&String::from(\"{}\"), 0", self.name).as_bytes())?,
            _ => file.write_all(format!("        let reply = self.dispatch(&String::from(\"{}\"), 0", self.name).as_bytes())?
        }

        if self.arguments.len() > 0 {
            for (key, _) in &self.arguments {
                file.write_all(format!(", &{}.as_encapsulation()?", key).as_bytes())?;
            }
            file.write_all(format!(")?;\n").as_bytes())?;
        } else {
            file.write_all(format!(", &Encapsulation::empty())?;\n").as_bytes())?;
        }
        match self.return_type {
            IceType::VoidType => {
                file.write_all(format!("        Ok(())\n").as_bytes())?;
            },
            _ => {
                file.write_all(format!("        {}::from_encapsulation(reply.body)\n", self.return_type.rust_type()).as_bytes())?;
            }
        };

        file.write_all(format!("    }}\n").as_bytes())?;
        Ok(())
    }
}