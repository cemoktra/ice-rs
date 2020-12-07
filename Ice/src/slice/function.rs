use crate::slice::types::IceType;
use crate::slice::writer;
use crate::errors::Error;
use std::fs::File;
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

    pub fn generate_decl(&self, file: &mut File) -> Result<(), Error> {
        writer::write(file, &format!("fn {}(&mut self", self.function_name()), 1)?;
        for (key, var_type) in &self.arguments {
            writer::write(file, &format!(", {}: &{}", snakecase::to_snake_case(key), var_type.rust_type()), 0)?;
        }
        writer::write(file, &format!(") -> Result<{}, Error>;\n", self.return_type.rust_type()), 1)
    }

    pub fn generate_impl(&self, file: &mut File) -> Result<(), Error> {
        writer::write(file, &format!("fn {}(&mut self", self.function_name()), 1)?;
        for (key, var_type) in &self.arguments {
            writer::write(file, &format!(", {}: &{}", snakecase::to_snake_case(key), var_type.rust_type()), 0)?;
        }
        writer::write(file, &format!(") -> Result<{}, Error> {{\n", self.return_type.rust_type()), 0)?;

        match self.return_type {
            IceType::VoidType => writer::write(file, &format!("self.dispatch(&String::from(\"{}\"), 0", self.name), 2)?,
            _ => writer::write(file, &format!("let reply = self.dispatch(&String::from(\"{}\"), 0", self.name), 2)?
        }

        if self.arguments.len() > 0 {
            for (key, _) in &self.arguments {
                writer::write(file, &format!(", &{}.as_encapsulation()?", key), 0)?;
            }
            writer::write(file, ")?;\n", 0)?;
        } else {
            writer::write(file, ", &Encapsulation::empty())?;\n", 0)?;
        }

        match self.return_type {
            IceType::VoidType => {
                writer::write(file, "Ok(())\n", 2)?;
            },
            _ => {
                writer::write(file, &format!("{}::from_encapsulation(reply.body)\n", self.return_type.rust_type()), 2)?;
            }
        };

        writer::write(file, "}\n", 1)
    }
}