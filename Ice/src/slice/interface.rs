use crate::slice::function::Function;
use crate::slice::writer;
use inflector::cases::classcase;
use writer::Writer;


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

    pub fn generate(&self, writer: &mut Writer, mod_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        writer.generate_trait_open(&self.class_name(), Some("IceObject"), 0)?;
        for function in &self.functions {
            function.generate_decl(writer)?;  
        }
        writer.generate_close_block(0)?;

        let prx_name = format!("{}Prx", self.class_name());
        writer.generate_struct_open(&prx_name, 0)?;
        writer.generate_struct_member("proxy", "Proxy", 1)?;
        writer.generate_struct_member("id", "String", 1)?;
        writer.generate_close_block(0)?;
        writer.blank_line()?;
        
        writer.generate_impl(Some("IceObject"), &prx_name, 0)?;

        writer.write(&format!("const TYPE_ID: &'static str = \"{}::{}\";\n", mod_path, self.name), 1)?;

        writer.generate_fn(
            false, 
            Some(String::from("T: 'static + std::fmt::Debug + std::fmt::Display + FromBytes")),
            "dispatch", 
            vec![
                String::from("&mut self"),
                String::from("op: &str"),
                String::from("mode: u8"),
                String::from("params: &Encapsulation"),
            ],
            Some("Result<ReplyData, Box<dyn std::error::Error>>"),
            true,
            1
        )?;
        writer.write(&format!("let req = self.proxy.create_request(&self.id, op, mode, params);\n"), 2)?;
        writer.write("self.proxy.make_request::<T>(&req)\n", 2)?;
        writer.generate_close_block(1)?;
        writer.generate_close_block(0)?;
        writer.blank_line()?;

        writer.generate_impl(Some(&self.class_name()), &prx_name, 0)?;
        for function in &self.functions {
            function.generate_impl(writer)?;  
        }
        writer.generate_close_block(0)?;
        writer.blank_line()?;

        writer.generate_impl(None, &prx_name, 0)?;
        writer.generate_fn(true, None, "checked_cast", vec![String::from("id: &str"), String::from("proxy: Proxy")], Some("Result<Self, Box<dyn std::error::Error>>"), true, 1)?;
        writer.write("let mut my_proxy = Self {\n", 2)?;
        writer.write("proxy: proxy,\n", 3)?;
        writer.write("id: String::from(id)\n", 3)?;
        writer.write("};\n", 2)?;
        writer.blank_line()?;

        writer.write("if !my_proxy.ice_is_a()? {\n", 2)?;
        writer.write("return Err(Box::new(ProtocolError {}));\n", 3)?;
        writer.generate_close_block(2)?;
        writer.write("Ok(my_proxy)\n", 2)?;

        writer.generate_close_block(1)?;
        writer.generate_close_block(0)?;
        writer.blank_line()?;
        Ok(())
    }
}
