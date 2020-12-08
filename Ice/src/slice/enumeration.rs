use crate::errors::Error;
use crate::slice::writer;
use std::fs::File;
use std::io::prelude::*;
use inflector::cases::classcase;


#[derive(Clone, Debug)]
pub struct Enum {
    name: String,
    variants: Vec<(String, i32)>,
    next_value: i32
}

impl Enum {
    pub fn new(name: &str) -> Enum {
        Enum {
            name: String::from(name),
            variants: vec![],
            next_value: 0
        }
    }

    pub fn class_name(&self) -> String {
        classcase::to_class_case(&self.name)
    }

    pub fn add_variant(&mut self, name: &str, value: Option<i32>) {
        let value = match value {
            Some(value) => {
                self.next_value = value + 1;
                value
            },
            None => {
                let value = self.next_value;
                self.next_value = value + 1;
                value
            }
        };
        self.variants.push((String::from(name), value));
    }

    pub fn write(&self, file: &mut File) -> Result<(), Error> {
        file.write_all("#[derive(Debug, Copy, Clone, TryFromPrimitive, PartialEq)]\n".as_bytes())?;
        file.write_all("#[repr(i32)]\n".as_bytes())?;
        file.write_all(format!("pub enum {} {{\n", self.class_name()).as_bytes())?;
        
        for (variant, index) in &self.variants {
            file.write_all(format!("    {} = {},\n", classcase::to_class_case(variant), index).as_bytes())?;
        }

        file.write_all("}\n\n".as_bytes())?;

        writer::write_to_bytes(file, &self.class_name(), vec![String::from("bytes.extend(IceSize{size: *self as i32}.to_bytes()?);\n")])?;

        file.write_all(format!("impl FromBytes for {} {{\n", self.class_name()).as_bytes())?;
        file.write_all("    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error> {\n".as_bytes())?;
        file.write_all("        let mut read = 0;\n".as_bytes())?;
        file.write_all("        let enum_value =  IceSize::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?.size;\n".as_bytes())?;
        file.write_all("        *read_bytes = *read_bytes + read;\n\n".as_bytes())?;
        file.write_all(format!("        match {}::try_from(enum_value) {{\n", self.class_name()).as_bytes())?;
        file.write_all("            Ok(enum_type) => Ok(enum_type),\n".as_bytes())?;
        file.write_all("            _ => Err(Error::DecodingError)\n".as_bytes())?;
        file.write_all("        }\n    }\n}\n\n".as_bytes())?;

        writer::write_encapsulation(file, &self.class_name())?;

        Ok(())
    }
}