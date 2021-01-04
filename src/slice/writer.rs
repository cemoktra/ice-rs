use std::fs::File;
use std::io::prelude::*;

pub struct Writer {
    file: File,
}

impl Writer {
    pub fn new(file: File) -> Writer {
        Writer {
            file: file
        }
    }

    pub fn write(&mut self, content: &str, indent: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.file.write_all(format!("{:width$}{}", "", content, width=(4 * indent)).as_bytes())?;
        Ok(())
    }

    pub fn blank_line(&mut self)  -> Result<(), Box<dyn std::error::Error>> {
        self.write("\n", 0)
    }

    pub fn generate_use(&mut self, crate_name: &str, indent: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.write(&format!(
            "use {};\n",
            crate_name
        ), indent)
    }

    pub fn generate_mod(&mut self, mod_name: &str, indent: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.write(&format!(
            "pub mod {};\n",
            mod_name
        ), indent)
    }

    pub fn generate_typedef(&mut self, id: &str, var_type: &str, indent: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.write(&format!(
            "type {} = {};\n",
            id,
            var_type
        ), indent)
    }

    pub fn generate_close_block(&mut self, indent: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.write("}\n", indent)
    }

    pub fn generate_enum_open(&mut self, id: &str, indent: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.write(&format!(
            "pub enum {} {{\n",
            id
        ), indent)
    }

    pub fn generate_struct_open(&mut self, id: &str, indent: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.write(&format!(
            "pub struct {} {{\n",
            id
        ), indent)
    }

    pub fn generate_trait_open(&mut self, id: &str, derived: Option<&str>, indent: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.write(&format!("pub trait {}", id), indent)?;
        if derived.is_some() {
            self.write(&format!(" : {}", derived.unwrap()), 0)?;
        }
        self.write("{\n", 0)
    }

    pub fn generate_struct_member(&mut self, id: &str, vartype: &str, indent: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.write(&format!(
            "pub {}: {},\n",
            id,
            vartype
        ), indent)
    }

    pub fn generate_enum_variant(&mut self, id: &str, value: i32, indent: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.write(&format!(
            "{} = {},\n",
            id,
            value
        ), indent)
    }

    pub fn generate_impl(&mut self, impl_trait: Option<&str>, object: &str, indent: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.write(&format!(
            "impl {}{} {} {{\n",
            if impl_trait.is_some() { impl_trait.unwrap() } else { "" },
            if impl_trait.is_some() { " for" } else { "" },
            object
        ), indent)
    }

    pub fn generate_fn(&mut self, public: bool, generic: Option<String>, fn_name: &str, arguments: Vec<String>, return_type: Option<&str>, is_impl: bool, indent: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.write(&format!(
            "{}fn {}",
            if public { "pub " } else { "" },
            fn_name
        ), indent)?;
        if generic.is_some() {
            self.write(&format!(
                "<{}>",
                generic.unwrap()
            ), 0)?;
        }
        self.write("(", 0)?;
        self.write(&arguments.join(", "), 0)?;
        self.write(")", 0)?;
        if return_type.is_some() {
            self.write(" -> ", 0)?;
            self.write(return_type.unwrap(), 0)?;
        }
        if is_impl {
            self.write(" {\n", 0)
        } else {
            self.write(";\n", 0)
        }
    }

    pub fn generate_to_bytes_impl(&mut self, object: &str, lines: Vec<String>, indent: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.generate_impl(Some("ToBytes"), object, indent)?;
        self.generate_fn(false, None, "to_bytes", vec![String::from("&self")], Some("Result<Vec<u8>, Box<dyn std::error::Error>>"), true, indent + 1)?;
        self.write("let mut bytes = Vec::new();\n", indent + 2)?;
        for line in lines {
            self.write(&line, indent + 2)?;
            self.blank_line()?;
        }
        self.write("Ok(bytes)\n", indent + 2)?;
        self.generate_close_block(1)?;
        self.generate_close_block(0)
    }

    pub fn generate_from_bytes_impl(&mut self, object: &str, lines: Vec<String>, pre_read: Option<Vec<String>>, indent: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.generate_impl(Some("FromBytes"), object, indent)?;
        self.generate_fn(false, None, "from_bytes", vec![String::from("bytes: &[u8]"), String::from("read_bytes: &mut i32")], Some("Result<Self, Box<dyn std::error::Error>>"), true, indent + 1)?;

        self.write("let mut read = 0;\n", indent + 2)?;

        if pre_read.is_some() {
            for item in pre_read.unwrap() {
                self.write(&item, indent + 2)?;
                self.blank_line()?;
            }
        }

        self.write("let obj = Self{\n", indent + 2)?;
        for line in lines {
            self.write(&line, indent + 3)?;
            self.blank_line()?;
        }
        self.write("};\n", indent + 2)?;
        self.write("*read_bytes = *read_bytes + read;\n", indent + 2)?;
        self.write("Ok(obj)\n", indent + 2)?;
        self.generate_close_block(1)?;
        self.generate_close_block(0)
    }

    pub fn generate_derive(&mut self, traits: Vec<&str>, indent: usize) -> Result<(), Box<dyn std::error::Error>> {
        self.write("#[derive(", indent)?;
        self.write(&traits.join(", "), indent)?;
        self.write(")]\n", indent)
    }
}

pub fn write(file: &mut File, line: &str, indent: usize) -> Result<(), Box<dyn std::error::Error>> {
    file.write_all(format!("{:width$}{}", "", line, width=(4 * indent)).as_bytes())?;
    Ok(())
}
