use crate::slice::types::IceType;
use crate::slice::writer;
use crate::slice::escape::escape;
use inflector::cases::snakecase;
use writer::Writer;


#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub return_type: IceType,
    pub arguments: Vec<(String, IceType, bool)>,
    pub throws: Option<IceType>,
    idempotent: bool,
    return_proxy: bool
}

impl Function {
    pub fn empty() -> Function {
        Function {
            name: String::new(),
            return_type: IceType::VoidType,
            arguments: Vec::new(),
            throws: None,
            idempotent: false,
            return_proxy: false
        }
    }

    pub fn new(name: &str, return_type: IceType) -> Function {
        Function {
            name: String::from(name),
            return_type: return_type,
            arguments: Vec::new(),
            throws: None,
            idempotent: false,
            return_proxy: false
        }
    }

    pub fn set_idempotent(&mut self) {
        self.idempotent = true;
    }

    pub fn set_return_proxy(&mut self) {
        self.return_proxy = true;
    }

    pub fn function_name(&self) -> String {
        snakecase::to_snake_case(&self.name)
    }

    pub fn add_argument(&mut self, name: &str, var_type: IceType, output: bool) {
        self.arguments.push((String::from(name), var_type, output));
    }

    pub fn set_throw(&mut self, throws: Option<IceType>) {
        self.throws = throws;
    }

    pub fn generate_decl(&self, writer: &mut Writer) -> Result<(), Box<dyn std::error::Error>> {        
        let mut arguments = Vec::new();
        arguments.push(String::from("&mut self"));
        for (key, var_type, out) in &self.arguments {
            arguments.push(
                format!(
                    "{}: {}{}{}",
                    escape(&snakecase::to_snake_case(key)),
                    if var_type.as_ref() | *out { "&" } else { "" },
                    if *out { "mut "} else { "" },
                    var_type.rust_type()
                )
            );
        }
        writer.generate_fn(
            false,
            None,
            &self.function_name(),
            arguments,
            Some(&format!(
                "Result<{}{}, Box<dyn std::error::Error>>",
                self.return_type.rust_type(),
                if self.return_proxy { "Prx" } else { "" }                
            )),
            false,
            1
        )
    }

    pub fn generate_impl(&self, writer: &mut Writer) -> Result<(), Box<dyn std::error::Error>> {
        let mut arguments = Vec::new();
        arguments.push(String::from("&mut self"));
        for (key, var_type, out) in &self.arguments {
            arguments.push(
                format!(
                    "{}: {}{}{}",
                    escape(&snakecase::to_snake_case(key)),  
                    if var_type.as_ref() | *out { "&" } else { "" },
                    if *out { "mut "} else { "" },
                    var_type.rust_type()
                )
            );
        }
        writer.generate_fn(
            false,
            None,
            &self.function_name(),
            arguments,
            Some(&format!(
                "Result<{}{}, Box<dyn std::error::Error>>",
                self.return_type.rust_type(),
                if self.return_proxy { "Prx" } else { "" }                
            )),
            true,
            1
        )?;

        
        let input_args_count = self.arguments.iter().filter(|(_, _, out)| !*out).count();
        let input_args = self.arguments.iter().filter(|(_, _, out)| !*out);
        let output_args_count = self.arguments.iter().filter(|(_, _, out)| *out).count();
        let output_args = self.arguments.iter().filter(|(_, _, out)| *out);
        writer.write(&format!("let {} bytes = Vec::new();\n", if input_args_count > 0 { "mut" } else { "" }), 2)?;
        for (key, ice_type, _) in input_args.into_iter() {
            match ice_type {
                IceType::Optional(var_type, tag) => {
                    writer.write(&format!("if let Some(value) = {} {{\n", escape(&snakecase::to_snake_case(key))), 2)?;
                    writer.write(&format!("bytes.extend(OptionalFlag::new({}, {}::optional_type()).to_bytes()?);\n", tag, var_type.rust_type()), 3)?;
                    writer.write("bytes.extend(value.to_bytes()?);\n", 3)?;
                    writer.write("}\n", 2)?;
                }
                _ => {
                    writer.write(&format!("bytes.extend({}.to_bytes()?);\n", escape(&snakecase::to_snake_case(key))), 2)?;
                }
            }
        }
        
        let mut require_reply = output_args_count > 0;
        match self.return_type {
            IceType::VoidType => {},
            _ => require_reply = true
        }

        let error_type = match &self.throws {
            Some(throws) => {
                throws.rust_type()
            },
            _ => {
                String::from("ProtocolError")
            }
        };
        if require_reply {
            writer.write(&format!("let reply = self.dispatch::<{}>(&String::from(\"{}\"), {}", error_type, self.name, if self.idempotent { 1 } else { 0 }), 2)?;
        } else {
            writer.write(&format!("self.dispatch::<{}>(&String::from(\"{}\"), {}", error_type, self.name, if self.idempotent { 1 } else { 0 }), 2)?;
        }
        writer.write(", &Encapsulation::from(bytes))?;\n\n", 0)?;

        if require_reply {
            writer.write("let mut read_bytes: i32 = 0;\n", 2)?;
            for (key, argtype, _) in output_args.into_iter() {
                writer.write(
            &format!(
                        "*{} = {}::from_bytes(&reply.body.data[read_bytes as usize..reply.body.data.len()], &mut read_bytes)?;\n",
                        snakecase::to_snake_case(key),
                        argtype.rust_from()
                    ),
                    2
                )?;
            }
        }
        
        match self.return_type {
            IceType::VoidType => {
                writer.write("Ok(())\n", 2)?;
            },
            _ => {
                match &self.return_type {
                    IceType::Optional(type_name, _) => {
                        writer.write(&format!("Option::<{}>::from_bytes(&reply.body.data[read_bytes as usize..reply.body.data.len()], &mut read_bytes)\n", type_name.rust_type()), 2)?;
                    }
                    IceType::CustomType(_) => {
                        if self.return_proxy {
                            writer.write("let proxy_data = ProxyData::from_bytes(&reply.body.data[read_bytes as usize..reply.body.data.len()], &mut read_bytes)?;\n", 2)?;
                            writer.write(&format!("let proxy_string = format!(\"{{}}:{{}} -h {{}} -p {{}}\", proxy_data.id, if proxy_data.secure {{ \"ssl\" }} else {{ \"tcp\" }}, self.proxy.host, self.proxy.port);\n"), 2)?;
                            writer.write("let comm = ice_rs::communicator::Communicator::new();\n", 2)?;
                            writer.write("let proxy = comm.string_to_proxy(&proxy_string)?;\n", 2)?;
                            writer.write("HelloPrx::checked_cast(proxy)", 2)?;
                        } else {
                            writer.write(&format!("{}::from_bytes(&reply.body.data[read_bytes as usize..reply.body.data.len()], &mut read_bytes)\n", self.return_type.rust_type()), 2)?;
                        }
                    }
                    _ => {
                        writer.write(&format!("{}::from_bytes(&reply.body.data[read_bytes as usize..reply.body.data.len()], &mut read_bytes)\n", self.return_type.rust_type()), 2)?;
                    }
                }                
            }
        };

        writer.generate_close_block(1)?;
        writer.blank_line()
    }
}