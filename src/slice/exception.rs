use crate::slice::types::IceType;
use crate::slice::writer;
use inflector::cases::{snakecase, pascalcase};
use writer::Writer;


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

    pub fn generate(&self, writer: &mut Writer) -> Result<(), Box<dyn std::error::Error>> {
        writer.generate_derive(vec!["Debug"], 0)?;
        writer.generate_struct_open(&self.class_name(), 0)?;
        for (type_name, var_type) in &self.members {
            writer.generate_struct_member(&snakecase::to_snake_case(type_name), &var_type.rust_type(), 1)?;
        }
        if self.extends.is_some() {
            writer.generate_struct_member("extends", &self.extends.as_ref().unwrap().rust_type(), 1)?;
        }
        writer.generate_close_block(0)?;
        writer.blank_line()?;

        writer.generate_impl(Some("std::fmt::Display"), &self.class_name(), 0)?;
        writer.generate_fn(false, None, "fmt", vec![String::from("&self"), String::from("f: &mut std::fmt::Formatter<'_>")], Some("std::fmt::Result"), true, 1)?;
        writer.write(&format!("write!(f, \"{}\")\n", self.class_name()), 2)?;
        writer.generate_close_block(1)?;
        writer.generate_close_block(0)?;
        writer.blank_line()?;

        writer.generate_impl(Some("std::error::Error"), &self.class_name(), 0)?;
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
        if self.extends.is_some() {
            lines.push(format!("extends:  {}::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?,", self.extends.as_ref().unwrap().rust_type()));
        }

        let pre_read = vec![
            String::from("let _flag = SliceFlags::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;"),
            String::from("let _slice_name = String::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;"),
        ];
        writer.generate_from_bytes_impl(&self.class_name(), lines, Some(pre_read), 0)
    }
}