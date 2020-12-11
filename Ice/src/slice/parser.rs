
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use pest::Parser;
use pest::iterators::Pairs;
use crate::errors::Error;
use crate::slice::module::Module;
use crate::slice::enumeration::Enum;
use crate::slice::struct_decl::Struct;
use crate::slice::interface::Interface;
use crate::slice::function::Function;
use crate::slice::types::IceType;


#[derive(Parser)]
#[grammar = "slice/ice.pest"]
pub struct IceParser;

pub trait ParsedObject {
    fn parse(rule: Pairs<Rule>) -> Result<Self, Error> where Self: Sized;
}

impl ParsedObject for Module {
    fn parse(rule: Pairs<Rule>) -> Result<Self, Error> where Self: Sized {
        let mut module = Module::new();
        for child in rule {
            match child.as_rule() {
                Rule::keyword_module => {},
                Rule::identifier => { 
                    module.name = String::from(child.as_str());
                    module.full_name = format!("::{}", module.name);
                },
                Rule::block_open => {},
                Rule::any_block => {
                    for block in child.into_inner() {
                        match block.as_rule() {
                            Rule::module_block => {
                                let mut sub_module = Module::parse(block.into_inner())?;
                                sub_module.full_name = format!("{}::{}", module.full_name, sub_module.name);
                                module.add_module(sub_module);
                            },
                            Rule::enum_block => {
                                let enumeration = Enum::parse(block.into_inner())?;
                                module.add_enum(enumeration);
                            },
                            Rule::struct_block => {
                                let structure = Struct::parse(block.into_inner())?;
                                module.add_struct(structure);
                            },
                            Rule::interface_block => {
                                let interface = Interface::parse(block.into_inner())?;
                                module.add_interface(interface);
                            },
                            _ => return Err(Error::ParsingError)
                        }
                    }
                },
                Rule::typedef => {
                    let mut vartype = IceType::VoidType;
                    let mut id = "";
                    for item in child.into_inner() {
                        match item.as_rule() {
                            Rule::typename => { vartype = IceType::from(item.as_str())? },
                            Rule::identifier => { id = item.as_str(); },
                            Rule::typedef_end => {
                                module.add_typedef(id, vartype.clone());
                            },
                            _ => return Err(Error::ParsingError)
                        }
                    }
                }
                Rule::block_close => {},
                _ => return Err(Error::ParsingError)
            };
        }
        Ok(module)
    }
}

impl ParsedObject for Enum {
    fn parse(rule: Pairs<Rule>) -> Result<Self, Error> where Self: Sized {
        let mut enumeration = Enum::empty();
        for child in rule {
            match child.as_rule() {
                Rule::keyword_enum => {},
                Rule::identifier => { enumeration.name = String::from(child.as_str()); },
                Rule::block_open => {},
                Rule::enum_lines => {
                    for line in child.into_inner() {
                        match line.as_rule() {
                            // TODO: maybe add struct for each enum line
                            Rule::enum_line => {
                                let mut id = "";
                                let mut value: Option<i32> = None;
                                for item in line.into_inner() {
                                    match item.as_rule() {
                                        Rule::identifier => {
                                            id = item.as_str();
                                        },
                                        Rule::numeric_value => {
                                            value = Some(item.as_str().parse()?);
                                        },
                                        _ => return Err(Error::ParsingError)
                                    };
                                }
                                enumeration.add_variant(id, value);
                            },
                            _ => return Err(Error::ParsingError)
                        }
                    }
                },
                Rule::block_close => {},
                _ => return Err(Error::ParsingError)
            }
        }

        Ok(enumeration)
    }
}

impl ParsedObject for Struct {
    fn parse(rule: Pairs<Rule>) -> Result<Self, Error> where Self: Sized {
        let mut structure = Struct::empty();
        for child in rule {            
            match child.as_rule() {
                Rule::keyword_struct => {},
                Rule::identifier => { structure.name = String::from(child.as_str()); },
                Rule::block_open => {},
                Rule::struct_line => {
                    let mut typename = IceType::VoidType;
                    let mut id = "";
                    for line in child.into_inner() {
                        match line.as_rule() {
                            Rule::typename => { typename = IceType::from(line.as_str())? },
                            Rule::identifier => { id = line.as_str(); },
                            Rule::struct_line_end => {
                                structure.add_member(id, typename.clone());
                            },
                            _ => return Err(Error::ParsingError)
                        }
                    }
                },
                Rule::block_close => {},
                _ => return Err(Error::ParsingError)
            }
        }

        Ok(structure)
    }
}

impl ParsedObject for Interface {
    fn parse(rule: Pairs<Rule>) -> Result<Self, Error> where Self: Sized {
        let mut interface = Interface::empty();
        for child in rule {
            match child.as_rule() {
                Rule::keyword_interface => {},
                Rule::identifier => { interface.name = String::from(child.as_str()); },
                Rule::block_open => {},
                Rule::function => {
                    interface.add_function(Function::parse(child.into_inner())?);
                },
                Rule::block_close => {},
                _ => return Err(Error::ParsingError)
            }
        }
        Ok(interface)
    }
}

impl ParsedObject for Function {
    fn parse(rule: Pairs<Rule>) -> Result<Self, Error> where Self: Sized {
        let mut function = Function::empty();
        for child in rule {
            match child.as_rule() {
                Rule::fn_return => { function.return_type = IceType::from(child.as_str())?; },
                Rule::fn_name => { function.name = String::from(child.as_str()); },
                Rule::fn_arg_open => {},
                Rule::fn_arg_list => {
                    for arg in child.into_inner() {
                        match arg.as_rule() {
                            Rule::fn_arg | Rule::fn_arg_out => {
                                let mut out = false;
                                let mut typename = IceType::VoidType;
                                let mut id = "";
                                for item in arg.into_inner() {                                    
                                    match item.as_rule() {
                                        Rule::typename => { typename = IceType::from(item.as_str())? },
                                        Rule::identifier => { id = item.as_str(); },
                                        Rule::keyword_out => { out = true; },
                                        _ => return Err(Error::ParsingError)
                                    }
                                }
                                function.add_argument(id, typename.clone(), out);
                            }
                            _ => return Err(Error::ParsingError)
                        }
                    }
                },
                Rule::fn_arg_close => {},
                Rule::fn_throws => {}
                _ => return Err(Error::ParsingError)
            }
        }
        Ok(function)
    }
}

impl Module {
    fn parse_file(file: &mut File) -> Result<Module, Error> {
        let mut root = Module::new();
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let pairs = IceParser::parse(Rule::ice, &content).unwrap();
        for pair in pairs {
            match pair.as_rule() {
                Rule::ice => {
                    for child in pair.into_inner() {
                        match child.as_rule() {
                            Rule::module_block => {
                                let module = Module::parse(child.into_inner())?;
                                root.add_module(module);
                            },
                            Rule::EOI => {
                                return Ok(root)
                            },
                            _ => return Err(Error::ParsingError)
                        }
                    }
                },
                _ => return Err(Error::ParsingError)
            };
        }

        Err(Error::ParsingError)
    }
}

impl<T> std::convert::From<pest::error::Error<T>> for Error {
    fn from(_err: pest::error::Error<T>) -> Error {
        Error::ParsingError
    }
}

impl std::convert::From<std::num::ParseIntError> for Error {
    fn from(_err: std::num::ParseIntError) -> Error {
        Error::ParsingError
    }
}

pub fn parse_ice_file(ice_file: &Path) -> Result<Module, Error> {
    let mut file = File::open(ice_file)?;
    let root = Module::parse_file(&mut file)?;
    Ok(root)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_function() {
        assert!(IceParser::parse(Rule::function, "void test();").is_ok());
        assert!(IceParser::parse(Rule::function, "long test();").is_ok());
        assert!(IceParser::parse(Rule::function, "long test(long width);").is_ok());
        assert!(IceParser::parse(Rule::function, "long test(long width, long height);").is_ok());
        assert!(IceParser::parse(Rule::function, "long test(long radius, out long area);").is_ok());
        assert!(IceParser::parse(Rule::function, "long test(out long area);").is_ok());
        assert!(IceParser::parse(Rule::function, "long test(out long area) throws exception;").is_ok());

        assert!(IceParser::parse(Rule::function, "123abc test();").is_err());
        assert!(IceParser::parse(Rule::function, "void 123abc();").is_err());
        assert!(IceParser::parse(Rule::function, "void test(123abc width);").is_err());
        assert!(IceParser::parse(Rule::function, "void test(long 123abc);").is_err());
        assert!(IceParser::parse(Rule::function, "void test(long width height);").is_err());
        assert!(IceParser::parse(Rule::function, "void test(out 123abc height);").is_err());
        assert!(IceParser::parse(Rule::function, "void test(out long 123abc);").is_err());
        assert!(IceParser::parse(Rule::function, "void test(out long result, long input);").is_err());
    }

    #[test]
    fn test_module_block() {
        assert!(IceParser::parse(Rule::ice, "#pragma once\nmodule Test { }").is_ok());

        assert!(IceParser::parse(Rule::module_block, "module Test { }").is_ok());
        assert!(IceParser::parse(Rule::module_block, "module Test { module Test2 {} }").is_ok());
        assert!(IceParser::parse(Rule::module_block, "module Test { enum Test2 { First } }").is_ok());
        assert!(IceParser::parse(Rule::module_block, "module Test { struct Test2 { long width; } }").is_ok());
        assert!(IceParser::parse(Rule::module_block, "module Test { interface Test2 { void test(); } }").is_ok());

        assert!(IceParser::parse(Rule::module_block, "module {}").is_err());
        assert!(IceParser::parse(Rule::module_block, "struct Test {}").is_err());
        assert!(IceParser::parse(Rule::module_block, "interface Test {}").is_err());
        assert!(IceParser::parse(Rule::module_block, "enum Test {}").is_err());
        assert!(IceParser::parse(Rule::module_block, "module 12Test {}").is_err());
    }

    #[test]
    fn test_enum_block() {
        assert!(IceParser::parse(Rule::enum_lines, "First = 0").is_ok());
        assert!(IceParser::parse(Rule::enum_lines, "First = 0, Second = 1").is_ok());
        assert!(IceParser::parse(Rule::enum_lines, "First").is_ok());
        assert!(IceParser::parse(Rule::enum_lines, "First, Second").is_ok());

        assert!(IceParser::parse(Rule::enum_block, "enum Test { First = 0 }").is_ok());
        assert!(IceParser::parse(Rule::enum_block, "enum Test { First }").is_ok());
        assert!(IceParser::parse(Rule::enum_block, "enum Test { First = 0, Second = 1 }").is_ok());
        assert!(IceParser::parse(Rule::enum_block, "enum Test { First, Second }").is_ok());

        assert!(IceParser::parse(Rule::enum_block, "struct Test {}").is_err());
        assert!(IceParser::parse(Rule::enum_block, "interface Test {}").is_err());
        assert!(IceParser::parse(Rule::enum_block, "module Test {}").is_err());
        assert!(IceParser::parse(Rule::enum_block, "enum 12Test {}").is_err());
        assert!(IceParser::parse(Rule::enum_block, "enum Test {}").is_err());
        assert!(IceParser::parse(Rule::enum_block, "enum Test { 123abc }").is_err());
        assert!(IceParser::parse(Rule::enum_block, "enum Test { 123abc = 1 }").is_err());
        assert!(IceParser::parse(Rule::enum_block, "enum Test { First = X }").is_err());
    }

    #[test]
    fn test_struct_block() {
        assert!(IceParser::parse(Rule::struct_block, "struct Test { long width; }").is_ok());
        assert!(IceParser::parse(Rule::struct_block, "struct Test { long width; long height; }").is_ok());

        assert!(IceParser::parse(Rule::struct_block, "struct Test { long width }").is_err());
        assert!(IceParser::parse(Rule::struct_block, "struct Test { }").is_err());
        assert!(IceParser::parse(Rule::struct_block, "enum Test {}").is_err());
        assert!(IceParser::parse(Rule::struct_block, "inteface Test {}").is_err());
        assert!(IceParser::parse(Rule::struct_block, "module Test {}").is_err());
        assert!(IceParser::parse(Rule::struct_block, "struct 12Test {}").is_err());
        assert!(IceParser::parse(Rule::struct_block, "struct Test { 12long width; }").is_err());
        assert!(IceParser::parse(Rule::struct_block, "struct Test { long 12width; }").is_err());
        assert!(IceParser::parse(Rule::struct_block, "struct Test { struct Test2 { } }").is_err());
    }

    #[test]
    fn test_interface_block() {
        assert!(IceParser::parse(Rule::interface_block, "interface Test { void test(); }").is_ok());
        assert!(IceParser::parse(Rule::interface_block, "interface Test { void test(long width); }").is_ok());
        assert!(IceParser::parse(Rule::interface_block, "interface Test { void test(long width, long height); }").is_ok());

        assert!(IceParser::parse(Rule::interface_block, "interface Test { void 12test(); }").is_err());
        assert!(IceParser::parse(Rule::interface_block, "interface Test { test(); }").is_err());
        assert!(IceParser::parse(Rule::interface_block, "interface Test { void test(long 12width); }").is_err());
        assert!(IceParser::parse(Rule::interface_block, "enum Test {}").is_err());
        assert!(IceParser::parse(Rule::interface_block, "struct Test {}").is_err());
        assert!(IceParser::parse(Rule::interface_block, "module Test {}").is_err());
    }
}