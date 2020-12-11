use crate::slice::types::IceType;
use crate::slice::writer;
use crate::errors::Error;
use std::fs::File;
use inflector::cases::snakecase;


#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub return_type: IceType,
    arguments: Vec<(String, IceType, bool)>
}

impl Function {
    pub fn empty() -> Function {
        Function {
            name: String::new(),
            return_type: IceType::VoidType,
            arguments: Vec::new()
        }
    }

    pub fn new(name: &str, return_type: IceType) -> Function {
        Function {
            name: String::from(name),
            return_type: return_type,
            arguments: Vec::new()
        }
    }

    pub fn function_name(&self) -> String {
        snakecase::to_snake_case(&self.name)
    }

    pub fn add_argument(&mut self, name: &str, var_type: IceType, output: bool) {
        self.arguments.push((String::from(name), var_type, output));
    }

    pub fn generate_decl(&self, file: &mut File) -> Result<(), Error> {
        writer::write(file, &format!("fn {}(&mut self", self.function_name()), 1)?;
        for (key, var_type, out) in &self.arguments {
            writer::write(
                file,
                &format!(
                    ", {}: {}{}{}",
                    snakecase::to_snake_case(key),                    
                    if var_type.as_ref() | *out { "&" } else { "" },
                    if *out { "mut "} else { "" },
                    var_type.rust_type()
                ),
                0
            )?;
        }
        writer::write(file, &format!(") -> Result<{}, Error>;\n", self.return_type.rust_type()), 0)
    }

    pub fn generate_impl(&self, file: &mut File) -> Result<(), Error> {
        writer::write(file, &format!("fn {}(&mut self", self.function_name()), 1)?;
        for (key, var_type, out) in &self.arguments {
            writer::write(
                file, 
                &format!(
                    ", {}: {}{}{}",
                    snakecase::to_snake_case(key),
                    if var_type.as_ref() | *out { "&" } else { "" },
                    if *out { "mut "} else { "" },
                    var_type.rust_type()
                ),
                0
            )?;
        }
        writer::write(file, &format!(") -> Result<{}, Error> {{\n", self.return_type.rust_type()), 0)?;

        let input_args_count = self.arguments.iter().filter(|(_, _, out)| !*out).count();
        let input_args = self.arguments.iter().filter(|(_, _, out)| !*out);
        let output_args_count = self.arguments.iter().filter(|(_, _, out)| *out).count();
        let output_args = self.arguments.iter().filter(|(_, _, out)| *out);
        writer::write(file, &format!("let {} bytes = Vec::new();\n", if input_args_count > 0 { "mut" } else { "" }), 2)?;
        for (key, _, _) in input_args.into_iter() {
            writer::write(file, &format!("bytes.extend({}.to_bytes()?);\n", key), 2)?;
        }
        
        let mut require_reply = output_args_count > 0;
        match self.return_type {
            IceType::VoidType => {},
            _ => require_reply = true
        }

        if require_reply {
            writer::write(file, &format!("let reply = self.dispatch(&String::from(\"{}\"), 0", self.name), 2)?;
        } else {
            writer::write(file, &format!("self.dispatch(&String::from(\"{}\"), 0", self.name), 2)?;
        }
        writer::write(file, ", &Encapsulation::from(bytes))?;\n\n", 0)?;

        if require_reply {
            writer::write(file, "let mut read_bytes: i32 = 0;\n", 2)?;
            for (key, argtype, _) in output_args.into_iter() {
                writer::write(
                    file,
                    &format!(
                        "*{} = {}::from_bytes(&reply.body.data[read_bytes as usize..reply.body.data.len()], &mut read_bytes)?;\n",
                        key,
                        argtype.rust_type()
                    ),
                    2
                )?;
            }
        }
        
        match self.return_type {
            IceType::VoidType => {
                writer::write(file, "Ok(())\n", 2)?;
            },
            _ => {
                writer::write(file, &format!("{}::from_bytes(&reply.body.data[read_bytes as usize..reply.body.data.len()], &mut read_bytes)\n", self.return_type.rust_type()), 2)?;
            }
        };

        writer::write(file, "}\n", 1)
    }
}