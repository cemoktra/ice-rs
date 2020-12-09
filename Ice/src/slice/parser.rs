
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use pest::Parser;
use pest::iterators::Pair;
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

fn parse_identifier(pair: Pair<Rule>) -> Result<String, Error> {
    let identifier = pair.into_inner().next().ok_or(Error::ParsingError)?;
    Ok(identifier.as_str().to_string())
}

fn parse_function_argument(pair: Pair<Rule>) -> Result<(IceType, String), Error> {
    let mut identifier = None;

    for child in pair.into_inner() {
        match child.as_rule() {
            Rule::identifier => {
                match identifier {
                    Some(identifier) => {
                        return Ok((IceType::from(identifier)?, child.as_str().to_string()));
                    },
                    _ => {
                        identifier = Some(child.as_str());
                    }
                }
            },
            _ => return Err(Error::ParsingError)
        };
    }
    Err(Error::ParsingError)
}

fn parse_function(pair: Pair<Rule>, interface: &mut Interface) -> Result<(), Error> {
    let mut fn_return = IceType::VoidType;
    let mut fn_name = String::new();
    let mut fn_args = Vec::new();

    for child in pair.into_inner() {
        match child.as_rule() {
            Rule::function_return => { fn_return = IceType::from(&parse_identifier(child)?)?; },
            Rule::function_name => { fn_name = parse_identifier(child)?; },
            Rule::function_argument => {
                fn_args.push(parse_function_argument(child)?);
            },
            Rule::function_end => {
                let mut function = Function::new(&fn_name, fn_return.clone());
                for (arg_type, arg_name) in &fn_args {
                    function.add_argument(&arg_name, arg_type.clone());
                }
                interface.add_function(function);
            },
            _ => return Err(Error::ParsingError)
        };
    }

    Ok(())
}

fn parse_struct_line(pair: Pair<Rule>, structure: &mut Struct) -> Result<(), Error> {
    let mut identifier = None;

    for child in pair.into_inner() {
        match child.as_rule() {
            Rule::identifier => {
                match identifier {
                    Some(identifier) => {
                        structure.add_member(child.as_str(), IceType::from(identifier)?);
                    },
                    _ => {
                        identifier = Some(child.as_str());
                    }
                }
            },
            _ => return Err(Error::ParsingError)
        };
    }
    Ok(())
}

fn parse_enum_line(pair: Pair<Rule>, enumeration: &mut Enum) -> Result<(), Error> {
    let mut last_type = None;
    for child in pair.into_inner() {
        match child.as_rule() {
            Rule::identifier => {
                match last_type {
                    Some(last) => {
                        enumeration.add_variant(last, None);
                    },
                    _ => {}
                }
                last_type = Some(child.as_str());
            },
            Rule::numeric_value => {
                let number:i32 = child.as_str().parse()?;
                enumeration.add_variant(last_type.ok_or(Error::ParsingError)?, Some(number));
                last_type = None;
            },
            _ => return Err(Error::ParsingError)
        }
    }
    match last_type {
        Some(last) => {
            enumeration.add_variant(last, None);
        },
        _ => {}
    }
    Ok(())
}

fn parse_interface(pair: Pair<Rule>, parent_module: &mut Module) -> Result<(), Error> {
    let mut inner = pair.into_inner();
    let identifier = inner.next().unwrap();
    if identifier.as_rule() != Rule::identifier {
        return Err(Error::ParsingError);
    }

    let mut interface = Interface::new(identifier.as_str());
    for child in inner {
        parse_function(child, &mut interface)?;
    }
    parent_module.add_interface(&interface);
    Ok(())
}

fn parse_struct(pair: Pair<Rule>, parent_module: &mut Module) -> Result<(), Error> {
    let mut inner = pair.into_inner();
    let identifier = inner.next().unwrap();
    if identifier.as_rule() != Rule::identifier {
        return Err(Error::ParsingError);
    }

    let mut structure = Struct::new(identifier.as_str());
    for child in inner {
        parse_struct_line(child, &mut structure)?;
    }
    parent_module.add_struct(&structure);

    Ok(())
}

fn parse_enum(pair: Pair<Rule>, parent_module: &mut Module) -> Result<(), Error> {
    let mut inner = pair.into_inner();
    let identifier = inner.next().unwrap();
    if identifier.as_rule() != Rule::identifier {
        return Err(Error::ParsingError);
    }

    let mut enumeration = Enum::new(identifier.as_str());
    for child in inner {
        parse_enum_line(child, &mut enumeration)?;
    }
    parent_module.add_enum(&enumeration);
    Ok(())
}

fn parse_module(pair: Pair<Rule>, parent_module: &mut Module) -> Result<(), Error> {
    let mut inner = pair.into_inner();
    let identifier = inner.next().unwrap();
    if identifier.as_rule() != Rule::identifier {
        return Err(Error::ParsingError);
    }

    let sub_module = parent_module.add_sub_module(identifier.as_str())?;

    for child in inner {
        parse_ice(child, sub_module)?;
    }
    Ok(())
}

fn parse_ice(pair: Pair<Rule>, parent_module: &mut Module) -> Result<(), Error> {
    match pair.as_rule() {
        Rule::ice | Rule::any_block => {
            parse_ice(pair.into_inner().next().unwrap(), parent_module)?;
        },
        Rule::module_block => {
            parse_module(pair, parent_module)?;
        },
        Rule::enum_block => {
            parse_enum(pair, parent_module)?;
        },
        Rule::struct_block => {
            parse_struct(pair, parent_module)?;
        },
        Rule::interface_block => {
            parse_interface(pair, parent_module)?;
        },
        _ => { return Err(Error::ParsingError); }
    }

    Ok(())
}

pub fn parse_ice_file(ice_file: &Path) -> Result<Module, Error> {
    let mut file = File::open(ice_file)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let mut root = Module::root();
    let slice = IceParser::parse(Rule::ice, &content)?.next().unwrap();
    parse_ice(slice, &mut root)?;

    Ok(root)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_module_block() {
        assert!(IceParser::parse(Rule::module_block, "module Test { }").is_ok());
        assert!(IceParser::parse(Rule::module_block, "module Test { module Test2 {} }").is_ok());
        assert!(IceParser::parse(Rule::module_block, "module Test { enum Test2 { First } }").is_ok());
        assert!(IceParser::parse(Rule::module_block, "module Test { struct Test2 { long width; } }").is_ok());
        assert!(IceParser::parse(Rule::module_block, "module Test { interface Test2 { void test(); } }").is_ok());

        assert!(IceParser::parse(Rule::module_block, "struct Test {}").is_err());
        assert!(IceParser::parse(Rule::module_block, "interface Test {}").is_err());
        assert!(IceParser::parse(Rule::module_block, "enum Test {}").is_err());
        assert!(IceParser::parse(Rule::module_block, "module 12Test {}").is_err());
    }

    #[test]
    fn test_enum_block() {
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