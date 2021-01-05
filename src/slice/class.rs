use crate::slice::types::IceType;
use crate::slice::writer;
use crate::slice::escape::escape;
use inflector::cases::{snakecase, pascalcase};
use writer::Writer;


#[derive(Clone, Debug)]
pub struct Class {
    pub name: String,
    members: Vec<(String, IceType)>
}

impl Class {
    pub fn empty() -> Class {
        Class {
            name: String::from(""),
            members: Vec::new()
        }
    }

    pub fn new(name: &str) -> Class {
        Class {
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
        writer.generate_derive(vec!["Debug", "Clone", "PartialEq"], 0)?;
        writer.generate_struct_open(&self.class_name(), 0)?;

        for (type_name, var_type) in &self.members {
            writer.generate_struct_member(&escape(&snakecase::to_snake_case(type_name)), &var_type.rust_type(), 1)?;
        }
        writer.generate_close_block(0)?;
        writer.blank_line()?;

        let mut lines = Vec::new();
        for (key, _) in &self.members {
            lines.push(format!("bytes.extend(self.{}.to_bytes()?);", &escape(&snakecase::to_snake_case(key))));
        }
        writer.generate_to_bytes_impl(&self.class_name(), lines, 0)?;

        // let mut lines = Vec::new();
        // for (key, var_type) in &self.members {
        //     lines.push(format!("{}:  {}::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?,", &escape(&snakecase::to_snake_case(key)), var_type.rust_from()));
        // }
        // writer.generate_from_bytes_impl(&self.class_name(), lines, None, 0)

        writer.generate_impl(Some("FromBytes"), &self.class_name(), 0)?;
        writer.generate_fn(false, None, "from_bytes", vec![String::from("bytes: &[u8]"), String::from("read_bytes: &mut i32")], Some("Result<Self, Box<dyn std::error::Error>>"), true, 1)?;

        writer.write("let mut read = 0;\n", 2)?;
        writer.write("let _marker = u8::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;\n", 2)?;
        writer.write("let _flags = SliceFlags::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;\n", 2)?;
        writer.write("let _slice_name = String::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;\n", 2)?;
        // TODO: check slice name matches class

        let mut has_optionals = false;
        for (key, var_type) in &self.members {
            match var_type {
                IceType::Optional(_, _) => {
                    has_optionals = true;
                    writer.write(&format!("let mut {} = None;\n", &escape(&snakecase::to_snake_case(key))), 2)?;
                }
                _ => {
                    writer.write(&format!("let {} = {}::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;\n", &escape(&snakecase::to_snake_case(key)), var_type.rust_from()), 2)?;
                }
            };
        }

        if has_optionals {
            writer.write("let mut flag_byte = bytes[read as usize..bytes.len()].first().unwrap();\n", 2)?;   
            writer.write("while *flag_byte != 0xFF as u8 {\n", 2)?;   
            writer.write("let flag = OptionalFlag::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?;\n", 3)?;
            writer.write("match flag.tag {\n", 3)?;

            for (key, var_type) in &self.members {
                match var_type {
                    IceType::Optional(type_name, tag) => {
                        writer.write(&format!("{} => {{\n", tag), 4)?;
                        // r#type = Some(NumberType::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?);
                        writer.write(&format!("{} = Some({}::from_bytes(&bytes[read as usize..bytes.len()], &mut read)?);\n", &escape(&snakecase::to_snake_case(key)), type_name.rust_type()), 5)?;
                        writer.generate_close_block(4)?;
                    }
                    _ => { }
                };
            }
            writer.write("_ => {\n", 4)?;
            writer.write("return Err(Box::new(ProtocolError {}));\n", 5)?;
            writer.generate_close_block(4)?;

            writer.generate_close_block(3)?;
            writer.write("flag_byte = bytes[read as usize..bytes.len()].first().unwrap();\n", 3)?;
            writer.generate_close_block(2)?;
        }

        writer.write("let obj = Self{\n", 2)?;
        for (key, _) in &self.members {
            writer.write(&format!("{}: {},\n", &escape(&snakecase::to_snake_case(key)), &escape(&snakecase::to_snake_case(key))), 3)?;
        }
        writer.write("};\n", 2)?;
        writer.write("*read_bytes = *read_bytes + read;\n", 2)?;
        writer.write("Ok(obj)\n", 2)?;
        
        writer.generate_close_block(1)?;
        writer.generate_close_block(0)?;
        writer.blank_line()
    }
}