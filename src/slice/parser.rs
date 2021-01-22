use std::collections::{BTreeSet, BTreeMap};
use std::path::Path;
use std::fs::File;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::prelude::*;
use inflector::cases::{classcase, pascalcase, snakecase};
use pest::Parser;
use pest::iterators::Pairs;
use quote::__private::TokenStream;
use quote::*;
use crate::errors::*;
use crate::slice::module::Module;
use crate::slice::enumeration::Enum;
use crate::slice::structure::Struct;
use crate::slice::interface::Interface;
use crate::slice::function::Function;
use crate::slice::exception::Exception;
use crate::slice::class::Class;
use crate::slice::types::IceType;

use super::{function_argument::FunctionArgument, function_return::FunctionReturn, function_throws::FunctionThrows, struct_member::StructMember};


#[derive(Parser)]
#[grammar = "slice/ice.pest"]
pub struct IceParser;

pub trait ParsedObject {
    fn parse(rule: Pairs<Rule>) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized;
}

pub trait ParsedModule {
    fn parse(&mut self, rule: &mut Pairs<Rule>) -> Result<(), Box<dyn std::error::Error>> where Self: Sized;
}


impl ParsedModule for Module {
    fn parse(&mut self, rule: &mut Pairs<Rule>) -> Result<(), Box<dyn std::error::Error>> where Self: Sized {
        let it = rule.next().ok_or(
            Box::new(ParsingError::new("No more items"))
        )?;
        if it.as_rule() != Rule::keyword_module {
            return Err(Box::new(ParsingError::new(
                &format!("Expected keyword module but found {:?}", it.as_rule())
            )));
        }
        let it = rule.next().ok_or(
            Box::new(ParsingError::new("No more items"))
        )?;
        if it.as_rule() != Rule::identifier {
            return Err(Box::new(ParsingError::new(
                &format!("Expected identifier but found {:?}", it.as_rule())
            )));
        }
        let name = it.as_str();
        let module = match self.sub_modules.iter_mut().find(|f| f.name == name) {
            Some(module) => {
                module
            }
            None => {
                let mut new_module = Module::new(Rc::clone(&self.type_map));
                new_module.name = String::from(name);
                new_module.full_name = format!("{}::{}", self.full_name, new_module.name);
                self.sub_modules.push(new_module);
                self.sub_modules.last_mut().ok_or(
                    Box::new(ParsingError::new("Unexpected. No module found."))
                )?
            }
        };

        for child in rule {
            match child.as_rule() {
                Rule::block_open => {},
                Rule::any_block => {
                    for block in child.into_inner() {
                        match block.as_rule() {
                            Rule::module_block => {
                                module.parse(&mut block.into_inner())?;
                            },
                            Rule::enum_block => {
                                let enumeration = Enum::parse(block.into_inner())?;
                                self.type_map.borrow_mut().insert(enumeration.id.to_string(), module.snake_name());
                                module.add_enum(enumeration);
                            },
                            Rule::struct_block => {
                                let structure = Struct::parse(block.into_inner())?;
                                self.type_map.borrow_mut().insert(structure.id.to_string(), module.snake_name());
                                module.add_struct(structure);
                            },
                            Rule::class_block => {
                                let class = Class::parse(block.into_inner())?;
                                self.type_map.borrow_mut().insert(class.id.to_string(), module.snake_name());
                                module.add_class(class);
                            },
                            Rule::interface_block => {
                                let interface = Interface::parse(block.into_inner())?;
                                module.add_interface(interface);
                            },
                            Rule::exception_block => {
                                let exception = Exception::parse(block.into_inner())?;
                                self.type_map.borrow_mut().insert(exception.id.to_string(), module.snake_name());
                                module.add_exception(exception);
                            }
                            _ => return Err(Box::new(ParsingError::new(
                                &format!("Unexpected rule {:?}", block.as_rule())
                            )))
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
                                self.type_map.borrow_mut().insert(String::from(id), module.snake_name());
                                module.add_typedef(id, vartype.clone());
                            },
                            _ => return Err(Box::new(ParsingError::new(
                                &format!("Unexpected rule {:?}", item.as_rule())
                            )))
                        }
                    }
                }
                Rule::block_close => {},
                _ => return Err(Box::new(ParsingError::new(
                    &format!("Unexpected rule {:?}", child.as_rule())
                )))
            };
        }
        Ok(())
    }
}

impl ParsedObject for Enum {
    fn parse(rule: Pairs<Rule>) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let mut enumeration = Enum::empty();
        for child in rule {
            match child.as_rule() {
                Rule::keyword_enum => {},
                Rule::identifier => { 
                    enumeration.ice_id = String::from(child.as_str());
                    let id_str = format_ident!("{}", classcase::to_class_case(&enumeration.ice_id));
                    enumeration.id = quote! { #id_str };
                },
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
                                        _ => return Err(Box::new(ParsingError::new(
                                            &format!("Unexpected rule {:?}", item.as_rule())
                                        )))
                                    };
                                }
                                enumeration.add_variant(id, value);
                            },
                            _ => return Err(Box::new(ParsingError::new(
                                &format!("Unexpected rule {:?}", line.as_rule())
                            )))
                        }
                    }
                },
                Rule::block_close => {},
                _ => return Err(Box::new(ParsingError::new(
                    &format!("Unexpected rule {:?}", child.as_rule())
                )))
            }
        }

        Ok(enumeration)
    }
}

impl ParsedObject for StructMember {
    fn parse(rule: Pairs<Rule>) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let mut member = StructMember::empty();
        let mut optional = false;
        let mut optional_tag = 0;
        for child in rule {
            match child.as_rule() {
                Rule::keyword_optional => {
                    optional = true;
                    for line in child.into_inner() {
                        match line.as_rule() {
                            Rule::optional_tag => {
                                optional_tag = line.as_str().parse()?;
                            }
                            _ => return Err(Box::new(ParsingError::new(
                                &format!("Unexpected rule {:?}", line.as_rule())
                            )))
                        }
                    }
                },
                Rule::typename => {
                    if optional {
                        member.r#type = IceType::Optional(Box::new(IceType::from(child.as_str())?), optional_tag);
                    } else {
                        member.r#type = IceType::from(child.as_str())?;
                    }
                },
                Rule::identifier => {
                    member.ice_id = String::from(child.as_str());
                    let id_str = format_ident!("{}", snakecase::to_snake_case(&member.ice_id));
                    member.id = quote! { #id_str };
                },
                Rule::struct_line_default => {
                    // TODO
                }
                Rule::struct_line_end => {
                },
                Rule::class_line_end => {
                },
                _ => return Err(Box::new(ParsingError::new(
                    &format!("Unexpected rule {:?}", child.as_rule())
                )))
            }
        }
        Ok(member)
    }
}

impl ParsedObject for Struct {
    fn parse(rule: Pairs<Rule>) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let mut structure = Struct::empty();
        for child in rule {
            match child.as_rule() {
                Rule::keyword_struct => {},
                Rule::identifier => { 
                    structure.ice_id = String::from(child.as_str());
                    let id_str = format_ident!("{}", classcase::to_class_case(&structure.ice_id));
                    structure.id = quote! { #id_str };
                },
                Rule::block_open => {},

                Rule::struct_line => {
                    let member = StructMember::parse(child.into_inner())?;
                    structure.add_member(member);
                },
                Rule::block_close => {},
                _ => return Err(Box::new(ParsingError::new(
                    &format!("Unexpected rule {:?}", child.as_rule())
                )))
            }
        }

        Ok(structure)
    }
}

impl ParsedObject for Class {
    fn parse(rule: Pairs<Rule>) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let mut class = Class::empty();
        for child in rule {
            match child.as_rule() {
                Rule::keyword_class => {},
                Rule::identifier => { 
                    class.ice_id = String::from(child.as_str());
                    let id_str = format_ident!("{}", classcase::to_class_case(&class.ice_id));
                    class.id = quote! { #id_str };
                },
                Rule::extends => {
                    for line in child.into_inner() {
                        match line.as_rule() {
                            Rule::keyword_extends => { },
                            Rule::identifier => {
                                class.extends = Some(IceType::from(line.as_str())?);
                            },
                            _ => return Err(Box::new(ParsingError::new(
                                &format!("Unexpected rule {:?}", line.as_rule())
                            )))
                        }
                    }
                }
                Rule::block_open => {},
                Rule::class_line => {
                    let member = StructMember::parse(child.into_inner())?;
                    class.add_member(member);
                },
                Rule::block_close => {},
                _ => return Err(Box::new(ParsingError::new(
                    &format!("Unexpected rule {:?}", child.as_rule())
                )))
            }
        }

        Ok(class)
    }
}

impl ParsedObject for Interface {
    fn parse(rule: Pairs<Rule>) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let mut interface = Interface::empty();
        for child in rule {
            match child.as_rule() {
                Rule::keyword_interface => {},
                Rule::identifier => { 
                    interface.ice_id = String::from(child.as_str());
                    let id_str = format_ident!("{}", classcase::to_class_case(&interface.ice_id));
                    interface.id = quote! { #id_str };
                },
                Rule::block_open => {},
                Rule::function => {
                    interface.add_function(Function::parse(child.into_inner())?);
                },
                Rule::block_close => {},
                _ => return Err(Box::new(ParsingError::new(
                    &format!("Unexpected rule {:?}", child.as_rule())
                )))
            }
        }
        Ok(interface)
    }
}

impl ParsedObject for Function {
    fn parse(rule: Pairs<Rule>) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let mut function = Function::empty();
        for child in rule {
            match child.as_rule() {
                Rule::keyword_idempotent => {
                    function.set_idempotent();
                }
                Rule::fn_return_proxy => {
                    function.return_type.set_proxy();
                }
                Rule::fn_return => {
                    function.return_type = FunctionReturn::parse(child.into_inner())?;
                },
                Rule::fn_name => { 
                    function.ice_id = String::from(child.as_str());
                    let id_str = format_ident!("{}", snakecase::to_snake_case(&function.ice_id));
                    function.id = quote! { #id_str };
                    
                },
                Rule::fn_arg_open => {},
                Rule::fn_arg_list => {
                    for arg in child.into_inner() {
                        match arg.as_rule() {
                            Rule::fn_arg | Rule::fn_arg_out => {
                                let arg = FunctionArgument::parse(arg.into_inner())?;
                                function.add_argument(arg);
                            }
                            _ => return Err(Box::new(ParsingError::new(
                                &format!("Unexpected rule {:?}", arg.as_rule())
                            )))
                        }
                    }
                },
                Rule::fn_arg_close => {},
                Rule::fn_throws => {
                    function.throws = FunctionThrows::parse(child.into_inner())?;
                }
                _ => return Err(Box::new(ParsingError::new(
                    &format!("Unexpected rule {:?}", child.as_rule())
                )))
            }
        }
        Ok(function)
    }
}

impl ParsedObject for FunctionThrows {
    fn parse(rule: Pairs<Rule>) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        for child in rule {
            match child.as_rule() {
                Rule::keyword_throws => {}
                Rule::identifier => {
                    return Ok(FunctionThrows::new(IceType::from(child.as_str())?));
                }
                _ => { }
            }
        }
        return Err(Box::new(ParsingError::new(
            &format!("Did not find throw identifier")
        )))
    }
}

impl ParsedObject for FunctionReturn {
    fn parse(rule: Pairs<Rule>) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let mut return_type = IceType::VoidType;
        let mut optional = false;
        let mut optional_tag = 0;
        for child in rule {
            match child.as_rule() {
                Rule::keyword_optional => {
                    optional = true;
                    let tag = child.into_inner().next().ok_or(Box::new(ParsingError::new("No more items")))?;

                    if tag.as_rule() != Rule::optional_tag {
                        return Err(Box::new(ParsingError::new(
                            &format!("Expected keyword optional_tag but found {:?}", tag.as_rule())
                        )));
                    }
                    optional_tag = tag.as_str().parse()?;
                }
                Rule::identifier => {
                    if optional {
                        return_type = IceType::Optional(Box::new(IceType::from(child.as_str())?), optional_tag);
                    } else {
                        return_type = IceType::from(child.as_str())?;
                    }
                }
                _ => return Err(Box::new(ParsingError::new(
                    &format!("Unexpected rule {:?}", child.as_rule())
                )))
            }
        }
        Ok(FunctionReturn::new(return_type))
    }
}

impl ParsedObject for FunctionArgument {
    fn parse(rule: Pairs<Rule>) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let mut id = TokenStream::new();
        let mut optional = false;
        let mut optional_tag = 0;
        let mut typename = IceType::VoidType;
        let mut out = false;

        for child in rule {
            match child.as_rule() {
                Rule::typename => {
                    if optional {
                        typename = IceType::Optional(Box::new(IceType::from(child.as_str())?), optional_tag);
                    } else {
                        typename = IceType::from(child.as_str())?;
                    }
                },
                Rule::identifier => {
                    let id_str = format_ident!("{}", snakecase::to_snake_case(child.as_str()));
                    id = quote! { #id_str }
                },
                Rule::keyword_out => out = true,
                Rule::keyword_optional => {
                    optional = true;
                    let tag = child.into_inner().next().ok_or(Box::new(ParsingError::new("No more items")))?;

                    if tag.as_rule() != Rule::optional_tag {
                        return Err(Box::new(ParsingError::new(
                            &format!("Expected keyword optional_tag but found {:?}", tag.as_rule())
                        )));
                    }
                    optional_tag = tag.as_str().parse()?;
                }
                _ => return Err(Box::new(ParsingError::new(
                    &format!("Unexpected rule {:?}", child.as_rule())
                )))
            }
        }
        Ok(FunctionArgument::new(id, typename, out))
    }
}

impl ParsedObject for Exception {
    fn parse(rule: Pairs<Rule>) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let mut exception = Exception::empty();
        for child in rule {
            match child.as_rule() {
                Rule::keyword_exception => {},
                Rule::identifier => { 
                    exception.ice_id = String::from(child.as_str());
                    let id_str = format_ident!("{}", pascalcase::to_pascal_case(&exception.ice_id));
                    exception.id = quote! { #id_str };
                },
                Rule::block_open => {},
                Rule::extends => {
                    for line in child.into_inner() {
                        match line.as_rule() {
                            Rule::keyword_extends => { },
                            Rule::identifier => {
                                exception.extends = Some(IceType::from(line.as_str())?);
                            },
                            _ => return Err(Box::new(ParsingError::new(
                                &format!("Unexpected rule {:?}", line.as_rule())
                            )))
                        }
                    }
                }
                Rule::struct_line => {
                    let member = StructMember::parse(child.into_inner())?;
                    exception.add_member(member);
                },
                Rule::block_close => {},
                _ => return Err(Box::new(ParsingError::new(
                    &format!("Unexpected rule {:?}", child.as_rule())
                )))
            }
        }

        Ok(exception)
    }
}

impl Module {
    fn parse_file(&mut self, file: &mut File, include_dir: &Path, parsed_files: &mut BTreeSet<String>) -> Result<(), Box<dyn std::error::Error>> {
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let pairs = IceParser::parse(Rule::ice, &content).unwrap();
        for pair in pairs {
            match pair.as_rule() {
                Rule::ice => {
                    for child in pair.into_inner() {
                        match child.as_rule() {
                            Rule::file_include => {
                                for item in child.into_inner() {
                                    match item.as_rule() {
                                        Rule::keyword_include => {},
                                        Rule::identifier => {
                                            let include = include_dir.join(format!("{}.ice", item.as_str()));
                                            let include_str = String::from(include_dir.join(format!("{}.ice", item.as_str())).to_str().unwrap());
                                            println!("  parsing include {} ... ", include_str);
                                            if parsed_files.contains(&include_str) {
                                                println!("  skip file!");
                                            } else {
                                                parsed_files.insert(include_str);
                                                let mut include_file = File::open(include)?;
                                                self.parse_file(&mut include_file, include_dir, parsed_files)?;
                                                println!("  finished include");
                                            }
                                        }
                                        _ => return Err(Box::new(ParsingError::new(
                                            &format!("Unexpected rule {:?}", item.as_rule())
                                        )))
                                    }
                                }
                            }
                            Rule::module_block => {
                                self.parse(&mut child.into_inner())?;
                            },
                            Rule::EOI => {
                                return Ok(())
                            },
                            _ => return Err(Box::new(ParsingError::new(
                                &format!("Unexpected rule {:?}", child.as_rule())
                            )))
                        }
                    }
                },
                _ => return Err(Box::new(ParsingError::new(
                    &format!("Unexpected rule {:?}", pair.as_rule())
                )))
            };
        }

        Err(Box::new(ParsingError::new("Unexpected error while parsing")))
    }
}

pub fn parse_ice_files(ice_files: &Vec<String>, include_dir: &str) -> Result<Module, Box<dyn std::error::Error>> {
    let mut parsed_files = BTreeSet::new();
    let mut root = Module::new(Rc::new(RefCell::new(BTreeMap::new())));
    for item in ice_files {
        println!("parsing {} ... ", item);
        if parsed_files.contains(item) {
            println!("skip file!");
        } else {
            parsed_files.insert(item.clone());
            let mut file = File::open(Path::new(&item))?;
            root.parse_file(&mut file, Path::new(include_dir), &mut parsed_files)?;
            println!("finished parsing!");
        }
    }

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