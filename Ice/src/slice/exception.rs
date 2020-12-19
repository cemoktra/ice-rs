use crate::slice::types::IceType;
use crate::slice::writer;
use std::fs::File;
use inflector::cases::{snakecase, pascalcase};


#[derive(Clone, Debug)]
pub struct Exception {
    pub name: String,
    pub extends: Option<IceType>,
    members: Vec<(String, IceType)>
}

impl Exception {
    pub fn empty() -> Exception {
        Exception {
            name: String::from(""),
            extends: None,
            members: Vec::new()
        }
    }

    pub fn add_member(&mut self, name: &str, var_type: IceType) {
        self.members.push((String::from(name), var_type));
    }

    pub fn class_name(&self) -> String {
        pascalcase::to_pascal_case(&self.name)
    }

    pub fn generate(&self, file: &mut File) -> Result<(), Box<dyn std::error::Error>> {
        writer::write(file, "#[derive(Debug)]\n", 0)?;
        writer::write(file, &format!("pub struct {} {{\n", self.class_name()), 0)?;

        for (type_name, var_type) in &self.members {
            writer::write(file, &format!("pub {}: {},\n", snakecase::to_snake_case(type_name), var_type.rust_type()), 1)?;
        }
        if self.extends.is_some() {
            writer::write(file, &format!("pub extends: {},\n", self.extends.as_ref().unwrap().rust_type()), 1)?;
        }

        writer::write(file, "}\n\n", 0)?;

        writer::write(file, &format!("impl std::fmt::Display for {} {{\n", self.class_name()), 0)?;
        writer::write(file, "fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {\n", 1)?;
        // TODO: create write for all members
        writer::write(file, &format!("write!(f, \"{}\")\n", self.class_name()), 2)?;
        
        writer::write(file, "}\n}\n\n", 1)?;

        writer::write(file, &format!("impl std::error::Error for {} {{}}\n\n", self.class_name()), 0)?;

        let mut lines = Vec::new();
        for (key, _) in &self.members {
            lines.push(format!("bytes.extend(self.{}.to_bytes()?);\n", snakecase::to_snake_case(key)));
        }
        writer::write_to_bytes(file, &self.class_name(), lines)?;

        let mut lines = Vec::new();
        for (key, var_type) in &self.members {
            lines.push(format!("{}:  {}::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?,\n", snakecase::to_snake_case(key), var_type.rust_type()));
        }
        if self.extends.is_some() {
            lines.push(format!("extends:  {}::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?,\n", self.extends.as_ref().unwrap().rust_type()));
        }

        writer::write_from_bytes_exception(file, &self.class_name(), lines)?;

        Ok(())
    }
}