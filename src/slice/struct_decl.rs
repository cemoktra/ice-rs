use crate::slice::types::IceType;
use crate::slice::writer;
use inflector::cases::{snakecase, pascalcase};
use writer::Writer;


#[derive(Clone, Debug)]
pub struct Struct {
    pub name: String,
    members: Vec<(String, IceType)>
}

impl Struct {
    pub fn empty() -> Struct {
        Struct {
            name: String::from(""),
            members: Vec::new()
        }
    }

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

    pub fn generate(&self, writer: &mut Writer) -> Result<(), Box<dyn std::error::Error>> {
        writer.generate_derive(vec!["Debug", "Copy", "Clone", "PartialEq"], 0)?;
        writer.generate_struct_open(&self.class_name(), 0)?;

        for (type_name, var_type) in &self.members {
            writer.generate_struct_member(&snakecase::to_snake_case(type_name), &var_type.rust_type(), 1)?;
        }
        writer.generate_close_block(0)?;
        writer.blank_line()?;

        let mut lines = Vec::new();
        for (key, _) in &self.members {
            lines.push(format!("bytes.extend(self.{}.to_bytes()?);", snakecase::to_snake_case(key)));
        }
        writer.generate_to_bytes_impl(&self.class_name(), lines, 0)?;

        let mut lines = Vec::new();
        for (key, var_type) in &self.members {
            lines.push(format!("{}:  {}::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?,", snakecase::to_snake_case(key), var_type.rust_type()));
        }
        writer.generate_from_bytes_impl(&self.class_name(), lines, None, 0)
    }
}