use std::collections::{BTreeSet, BTreeMap};
use std::path::Path;
use std::fs::File;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::prelude::*;
use pest::Parser;
use pest::iterators::Pairs;
use crate::errors::*;
use crate::slice::module::Module;
use crate::slice::enumeration::Enum;
use crate::slice::structure::Struct;
use crate::slice::interface::Interface;
use crate::slice::function::Function;
use crate::slice::exception::Exception;
use crate::slice::class::Class;
use crate::slice::types::IceType;


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
        let it = rule.next().ok_or(Box::new(ParsingError {}))?;
        if it.as_rule() != Rule::keyword_module {
            return Err(Box::new(ParsingError {}));
        }
        let it = rule.next().ok_or(Box::new(ParsingError {}))?;
        if it.as_rule() != Rule::identifier {
            return Err(Box::new(ParsingError {}));
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
                self.sub_modules.last_mut().ok_or(Box::new(ParsingError {}))?
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
                                self.type_map.borrow_mut().insert(enumeration.class_name(), module.snake_name());
                                module.add_enum(enumeration);
                            },
                            Rule::struct_block => {
                                let structure = Struct::parse(block.into_inner())?;
                                self.type_map.borrow_mut().insert(structure.class_name(), module.snake_name());
                                module.add_struct(structure);
                            },
                            Rule::class_block => {
                                let class = Class::parse(block.into_inner())?;
                                self.type_map.borrow_mut().insert(class.class_name(), module.snake_name());
                                module.add_class(class);
                            },
                            Rule::interface_block => {
                                let interface = Interface::parse(block.into_inner())?;
                                module.add_interface(interface);
                            },
                            Rule::exception_block => {
                                let exception = Exception::parse(block.into_inner())?;
                                module.add_exception(exception);
                            }
                            _ => return Err(Box::new(ParsingError {}))
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
                            _ => return Err(Box::new(ParsingError {}))
                        }
                    }
                }
                Rule::block_close => {},
                _ => return Err(Box::new(ParsingError {}))
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
                                        _ => return Err(Box::new(ParsingError {}))
                                    };
                                }
                                enumeration.add_variant(id, value);
                            },
                            _ => return Err(Box::new(ParsingError {}))
                        }
                    }
                },
                Rule::block_close => {},
                _ => return Err(Box::new(ParsingError {}))
            }
        }

        Ok(enumeration)
    }
}

impl ParsedObject for Struct {
    fn parse(rule: Pairs<Rule>) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
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
                            Rule::typename => { 
                                typename = IceType::from(line.as_str())?;
                            },
                            Rule::identifier => { 
                                id = line.as_str();                                
                            },
                            Rule::struct_line_default => {
                                // TODO
                                structure.add_member(id, typename.clone());
                            }
                            Rule::struct_line_end => {
                                structure.add_member(id, typename.clone());
                            },
                            _ => return Err(Box::new(ParsingError {}))
                        }
                    }
                },
                Rule::block_close => {},
                _ => return Err(Box::new(ParsingError {}))
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
                Rule::identifier => { class.name = String::from(child.as_str()); },
                Rule::extends => {
                    for line in child.into_inner() {
                        match line.as_rule() {
                            Rule::keyword_extends => { },
                            Rule::identifier => {
                                class.extends = Some(IceType::from(line.as_str())?);
                            },
                            _ => return Err(Box::new(ParsingError {}))
                        }
                    }
                }
                Rule::block_open => {},
                Rule::class_line => {
                    let mut optional = false;
                    let mut optional_tag = 0;
                    let mut typename = IceType::VoidType;
                    let mut id = "";
                    for line in child.into_inner() {
                        match line.as_rule() {
                            Rule::keyword_optional => {
                                optional = true;
                                for line in line.into_inner() {
                                    match line.as_rule() {
                                        Rule::optional_tag => {
                                            optional_tag = line.as_str().parse()?;
                                        }
                                        _ => return Err(Box::new(ParsingError {}))
                                    }
                                }
                            }
                            Rule::typename => {
                                if optional {
                                    typename = IceType::Optional(Box::new(IceType::from(line.as_str())?), optional_tag);
                                } else {
                                    typename = IceType::from(line.as_str())?;
                                }
                            },
                            Rule::identifier => {
                                id = line.as_str();
                            },
                            Rule::class_line_default => {
                                // TODO
                                class.add_member(id, typename.clone());
                            }
                            Rule::class_line_end => {
                                class.add_member(id, typename.clone());
                            },
                            _ => return Err(Box::new(ParsingError {}))
                        }
                    }
                },
                Rule::block_close => {},
                _ => return Err(Box::new(ParsingError {}))
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
                Rule::identifier => { interface.name = String::from(child.as_str()); },
                Rule::block_open => {},
                Rule::function => {
                    interface.add_function(Function::parse(child.into_inner())?);
                },
                Rule::block_close => {},
                _ => return Err(Box::new(ParsingError {}))
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
                Rule::fn_return => {
                    let mut optional = false;
                    let mut optional_tag = 0;
                    for arg in child.into_inner() {
                        match arg.as_rule() {
                            Rule::keyword_optional => {
                                optional = true;
                                for line in arg.into_inner() {
                                    match line.as_rule() {
                                        Rule::optional_tag => {
                                            optional_tag = line.as_str().parse()?;
                                        }
                                        _ => return Err(Box::new(ParsingError {}))
                                    }
                                }
                            }
                            Rule::identifier => {
                                if optional {
                                    function.return_type = IceType::Optional(Box::new(IceType::from(arg.as_str())?), optional_tag);
                                } else {
                                    function.return_type = IceType::from(arg.as_str())?;
                                }
                            }
                            _ => return Err(Box::new(ParsingError {}))
                        }
                    }
                },
                Rule::fn_name => { function.name = String::from(child.as_str()); },
                Rule::fn_arg_open => {},
                Rule::fn_arg_list => {
                    for arg in child.into_inner() {
                        match arg.as_rule() {
                            Rule::fn_arg | Rule::fn_arg_out => {
                                let mut out = false;
                                let mut optional = false;
                                let mut optional_tag = 0;
                                let mut typename = IceType::VoidType;
                                let mut id = "";
                                for item in arg.into_inner() {
                                    match item.as_rule() {
                                        Rule::typename => {
                                            if optional {
                                                typename = IceType::Optional(Box::new(IceType::from(item.as_str())?), optional_tag);
                                            } else {
                                                typename = IceType::from(item.as_str())?;
                                            }
                                        },
                                        Rule::identifier => { id = item.as_str(); },
                                        Rule::keyword_out => { out = true; },
                                        Rule::keyword_optional => {
                                            optional = true;
                                            for line in item.into_inner() {
                                                match line.as_rule() {
                                                    Rule::optional_tag => {
                                                        optional_tag = line.as_str().parse()?;
                                                    }
                                                    _ => return Err(Box::new(ParsingError {}))
                                                }
                                            }
                                        }
                                        _ => return Err(Box::new(ParsingError {}))
                                    }
                                }
                                function.add_argument(id, typename.clone(), out);
                            }
                            _ => return Err(Box::new(ParsingError {}))
                        }
                    }
                },
                Rule::fn_arg_close => {},
                Rule::fn_throws => {
                    for arg in child.into_inner() {
                        match arg.as_rule() {
                            Rule::keyword_throws => {}
                            Rule::identifier => {
                                function.set_throw(Some(IceType::from(arg.as_str())?));
                            }
                            _ => return Err(Box::new(ParsingError {}))
                        }
                    }
                }
                _ => return Err(Box::new(ParsingError {}))
            }
        }
        Ok(function)
    }
}

impl ParsedObject for Exception {
    fn parse(rule: Pairs<Rule>) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let mut exception = Exception::empty();
        for child in rule {
            match child.as_rule() {
                Rule::keyword_exception => {},
                Rule::identifier => { exception.name = String::from(child.as_str()); },
                Rule::block_open => {},
                Rule::extends => {
                    for line in child.into_inner() {
                        match line.as_rule() {
                            Rule::keyword_extends => { },
                            Rule::identifier => {
                                exception.extends = Some(IceType::from(line.as_str())?);
                            },
                            _ => return Err(Box::new(ParsingError {}))
                        }
                    }
                }
                Rule::struct_line => {
                    let mut typename = IceType::VoidType;
                    let mut id = "";
                    for line in child.into_inner() {
                        match line.as_rule() {
                            Rule::typename => { typename = IceType::from(line.as_str())? },
                            Rule::identifier => { id = line.as_str(); },
                            Rule::struct_line_end => {
                                exception.add_member(id, typename.clone());
                            },
                            _ => return Err(Box::new(ParsingError {}))
                        }
                    }
                },
                Rule::block_close => {},
                _ => return Err(Box::new(ParsingError {}))
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
                                        _ => return Err(Box::new(ParsingError {}))
                                    }
                                }
                            }
                            Rule::module_block => {
                                self.parse(&mut child.into_inner())?;
                            },
                            Rule::EOI => {
                                return Ok(())
                            },
                            _ => return Err(Box::new(ParsingError {}))
                        }
                    }
                },
                _ => return Err(Box::new(ParsingError {}))
            };
        }

        Err(Box::new(ParsingError {}))
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