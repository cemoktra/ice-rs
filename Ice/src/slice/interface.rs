use crate::slice::function::Function;
use crate::errors::Error;
use std::fs::File;
use std::io::prelude::*;
use inflector::cases::classcase;


#[derive(Clone)]
pub struct Interface {
    name: String,
    functions: Vec<Function>
}

impl Interface {
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

    pub fn write(&self, file: &mut File, mod_path: &str, context: &str) -> Result<(), Error> {
        file.write_all(format!("pub trait {} : IceObject {{\n", self.class_name()).as_bytes())?;
        for function in &self.functions {
            function.write_decl(file)?;  
        }
        file.write_all("}\n\n".as_bytes())?;

        file.write_all(format!("pub struct {}Prx {{\n", self.class_name()).as_bytes())?;
        file.write_all("    proxy: Proxy\n".as_bytes())?;
        file.write_all("}\n\n".as_bytes())?;

        file.write_all(format!("impl IceObject for {}Prx {{\n", self.class_name()).as_bytes())?;
        file.write_all(format!("    const TYPE_ID: &'static str = \"{}::{}\";\n", mod_path, self.name).as_bytes())?;
        file.write_all(format!("    const NAME: &'static str = \"{}\";\n\n", context).as_bytes())?;
        file.write_all(format!("    fn dispatch(&mut self, op: &str, mode: u8, params: &Encapsulation) -> Result<ReplyData, Error> {{\n").as_bytes())?;
        file.write_all(format!("        let req = self.proxy.create_request(&{}Prx::NAME, op, mode, params);\n", self.class_name()).as_bytes())?;
        file.write_all(format!("        self.proxy.make_request(&req)\n").as_bytes())?;
        file.write_all(format!("    }}\n\n").as_bytes())?;
        file.write_all("}\n\n".as_bytes())?;

        file.write_all(format!("impl {} for {}Prx {{\n", self.class_name(), self.class_name()).as_bytes())?;
        for function in &self.functions {
            function.write_impl(file)?;  
        }
        file.write_all("}\n\n".as_bytes())?;

        file.write_all(format!("impl {}Prx {{\n", self.class_name()).as_bytes())?;
        file.write_all(format!("    pub fn checked_cast(proxy: Proxy) -> Result<DemoPrx, Error> {{\n").as_bytes())?;
        file.write_all(format!("        let mut demo_proxy = DemoPrx {{\n").as_bytes())?;
        file.write_all(format!("            proxy: proxy\n").as_bytes())?;
        file.write_all(format!("        }};\n\n").as_bytes())?;
        file.write_all(format!("        if !demo_proxy.ice_is_a()? {{\n").as_bytes())?;
        file.write_all(format!("            return Err(Error::TcpError);\n").as_bytes())?;
        file.write_all(format!("        }}\n\n").as_bytes())?;
        file.write_all(format!("        Ok(demo_proxy)\n").as_bytes())?;
        file.write_all("    }\n}\n\n".as_bytes())?;

        Ok(())
    }
}
