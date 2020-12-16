use crate::slice::function::Function;
use crate::slice::writer;
use std::fs::File;
use inflector::cases::classcase;


#[derive(Clone, Debug)]
pub struct Interface {
    pub name: String,
    functions: Vec<Function>
}

impl Interface {
    pub fn empty() -> Interface {
        Interface {
            name: String::from(""),
            functions: Vec::new()
        }
    }

    pub fn new(name: &str) -> Interface {
        Interface {
            name: String::from(name),
            functions: Vec::new()
        }
    }

    pub fn class_name(&self) -> String {
        classcase::to_class_case(&self.name)
    }

    pub fn add_function(&mut self, function: Function) {
        self.functions.push(function);
    }

    pub fn generate(&self, file: &mut File, mod_path: &str, context: &str) -> Result<(), Box<dyn std::error::Error>> {
        writer::write(file, &format!("pub trait {} : IceObject {{\n", self.class_name()), 0)?;
        for function in &self.functions {
            function.generate_decl(file)?;  
        }
        writer::write(file, "}\n\n", 0)?;

        writer::write(file, &format!("pub struct {}Prx {{\n", self.class_name()), 0)?;
        writer::write(file, "proxy: Proxy\n}\n\n", 1)?;
        
        writer::write(file, &format!("impl IceObject for {}Prx {{\n", self.class_name()), 0)?;
        writer::write(file, &format!("const TYPE_ID: &'static str = \"{}::{}\";\n", mod_path, self.name), 1)?;
        writer::write(file, &format!("const NAME: &'static str = \"{}\";\n\n", context), 1)?;
        writer::write(file, "fn dispatch(&mut self, op: &str, mode: u8, params: &Encapsulation) -> Result<ReplyData, Box<dyn std::error::Error>> {\n", 1)?;
        writer::write(file, &format!("let req = self.proxy.create_request(&{}Prx::NAME, op, mode, params);\n", self.class_name()), 2)?;
        writer::write(file, "self.proxy.make_request(&req)\n", 2)?;
        writer::write(file, "}\n}\n\n", 1)?;

        writer::write(file, &format!("impl {} for {}Prx {{\n", self.class_name(), self.class_name()), 0)?;
        for function in &self.functions {
            function.generate_impl(file)?;  
        }
        writer::write(file, "}\n\n", 0)?;

        writer::write(file, &format!("impl {}Prx {{\n", self.class_name()), 0)?;
        writer::write(file, "pub fn checked_cast(proxy: Proxy) -> Result<Self, Box<dyn std::error::Error>> {\n", 1)?;
        writer::write(file, "let mut my_proxy = Self {\n", 2)?;
        writer::write(file, "proxy: proxy\n", 3)?;
        writer::write(file, "};\n\n", 2)?;
        writer::write(file, "if !my_proxy.ice_is_a()? {\n", 2)?;
        writer::write(file, "return Err(Box::new(ProtocolError {}));\n", 3)?;
        writer::write(file, "}\n\n", 2)?;
        writer::write(file, "Ok(my_proxy)\n", 2)?;
        writer::write(file, "}\n}\n\n", 1)
    }
}
