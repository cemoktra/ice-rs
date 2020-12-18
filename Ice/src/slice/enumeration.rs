use crate::slice::writer;
use std::fs::File;
use inflector::cases::classcase;


#[derive(Clone, Debug)]
pub struct Enum {
    pub name: String,
    variants: Vec<(String, i32)>,
    next_value: i32
}

impl Enum {
    pub fn empty() -> Enum {
        Enum {
            name: String::from(""),
            variants: vec![],
            next_value: 0
        }
    }

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

    pub fn generate(&self, file: &mut File) -> Result<(), Box<dyn std::error::Error>> {
        writer::write(file, "#[derive(Debug, Copy, Clone, TryFromPrimitive, PartialEq)]\n", 0)?;
        writer::write(file, "#[repr(i32)]\n", 0)?;
        writer::write(file, &format!("pub enum {} {{\n", self.class_name()), 0)?;
       
        for (variant, index) in &self.variants {
            writer::write(file, &format!("{} = {},\n", classcase::to_class_case(variant), index), 1)?;
        }
        writer::write(file, "}\n\n", 0)?;

        writer::write_to_bytes(file, &self.class_name(), vec![String::from("bytes.extend(IceSize{size: *self as i32}.to_bytes()?);\n")])?;

        writer::write(file, &format!("impl FromBytes for {} {{\n", self.class_name()), 0)?;
        writer::write(file, "fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Box<dyn std::error::Error>> {\n", 1)?;
        writer::write(file, "let mut read = 0;\n", 2)?;
        writer::write(file, "let enum_value =  IceSize::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?.size;\n", 2)?;
        writer::write(file, "*read_bytes = *read_bytes + read;\n\n", 2)?;
        writer::write(file, &format!("match {}::try_from(enum_value) {{\n", self.class_name()), 2)?;
        writer::write(file, "Ok(enum_type) => Ok(enum_type),\n", 3)?;
        writer::write(file, "_ => Err(Box::new(ProtocolError {}))\n", 3)?;
        writer::write(file, "}\n", 2)?;
        writer::write(file, "}\n}\n\n", 1)?;

        Ok(())
    }
}