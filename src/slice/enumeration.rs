use crate::slice::writer;
use inflector::cases::classcase;
use writer::Writer;


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

    pub fn generate(&self, writer: &mut Writer) -> Result<(), Box<dyn std::error::Error>> {
        writer.generate_derive(vec!["Debug", "Copy", "Clone", "TryFromPrimitive", "PartialEq"], 0)?;
        writer.write("#[repr(i32)]\n", 0)?;
        writer.generate_enum_open(&self.class_name(), 0)?;
        for (variant, index) in &self.variants {
            writer.generate_enum_variant(variant, *index, 1)?;
        }
        writer.generate_close_block(0)?;
        writer.blank_line()?;

        writer.generate_optional_type(
            &self.class_name(),
            4,
            0
        )?;

        writer.generate_to_bytes_impl(
            &self.class_name(),
            vec![String::from("bytes.extend(IceSize{size: *self as i32}.to_bytes()?);")],
            0
        )?;

        writer.generate_impl(Some("FromBytes"), &self.class_name(), 0)?;
        writer.generate_fn(false, None, "from_bytes", vec![String::from("bytes: &[u8]"), String::from("read_bytes: &mut i32")], Some("Result<Self, Box<dyn std::error::Error>>"), true, 1)?;
        writer.write("let mut read = 0;\n", 2)?;
        writer.write("let enum_value =  IceSize::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?.size;\n", 2)?;
        writer.write("*read_bytes = *read_bytes + read;\n\n", 2)?;
        writer.write(&format!("match {}::try_from(enum_value) {{\n", self.class_name()), 2)?;
        writer.write("Ok(enum_type) => Ok(enum_type),\n", 3)?;
        writer.write("_ => Err(Box::new(ProtocolError {}))\n", 3)?;
        writer.generate_close_block(2)?;
        writer.generate_close_block(1)?;
        writer.generate_close_block(0)
    }
}