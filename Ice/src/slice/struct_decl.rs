use crate::errors::Error;
use crate::slice::types::IceType;
use crate::slice::writer;
use std::fs::File;
use std::io::prelude::*;
use inflector::cases::{snakecase, pascalcase};


#[derive(Clone, Debug)]
pub struct Struct {
    name: String,
    members: Vec<(String, IceType)>
}

impl Struct {
    pub fn new(name: &str) -> Struct {
        Struct {
            name: String::from(name),
            members: Vec::new()
        }
    }

    pub fn add_member(&mut self, name: &str, var_type: IceType) {
        self.members.push((String::from(name), var_type));
    }

    pub fn class_name(&self) -> String {
        pascalcase::to_pascal_case(&self.name)
    }

    pub fn write(&self, file: &mut File) -> Result<(), Error> {
        println!("write struct {} {}", self.class_name(), self.name);
        file.write_all("#[derive(Debug, Copy, Clone, PartialEq)]\n".as_bytes())?;
        file.write_all(format!("pub struct {} {{\n", self.class_name()).as_bytes())?;
        
        for (type_name, var_type) in &self.members {
            file.write_all(format!("    pub {}: {},\n", snakecase::to_snake_case(type_name), var_type.rust_type()).as_bytes())?;
        }

        file.write_all("}\n\n".as_bytes())?;


        let mut lines = Vec::new();
        for (key, _) in &self.members {
            lines.push(format!("bytes.extend(self.{}.to_bytes()?);\n", snakecase::to_snake_case(key)));
        }
        writer::write_to_bytes(file, &self.class_name(), lines)?;

        let mut lines = Vec::new();
        for (key, var_type) in &self.members {
            lines.push(format!("{}:  {}::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?,\n", snakecase::to_snake_case(key), var_type.rust_type()));
        }
        writer::write_from_bytes(file, &self.class_name(), lines)?;

        writer::write_encapsulation(file, &self.class_name())?;

        Ok(())
    }
}