
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

fn parse_typename(pair: Pair<Rule>) -> Result<String, Error> {
    let typename = pair.into_inner().next().ok_or(Error::ParsingError)?;
    Ok(typename.as_str().to_string())
}

fn parse_function_argument(pair: Pair<Rule>) -> Result<(IceType, String), Error> {
    let mut typename = None;

    for child in pair.into_inner() {
        match child.as_rule() {
            Rule::typename => {
                match typename {
                    Some(typename) => {
                        return Ok((IceType::from(typename)?, child.as_str().to_string()));
                    },
                    _ => {
                        typename = Some(child.as_str());
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
            Rule::function_return => { fn_return = IceType::from(&parse_typename(child)?)?; },
            Rule::function_name => { fn_name = parse_typename(child)?; },
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
    let mut typename = None;

    for child in pair.into_inner() {
        match child.as_rule() {
            Rule::typename => {
                match typename {
                    Some(typename) => {
                        structure.add_member(child.as_str(), IceType::from(typename)?);
                    },
                    _ => {
                        typename = Some(child.as_str());
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
            Rule::typename => {
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
    let typename = inner.next().unwrap();
    if typename.as_rule() != Rule::typename {
        return Err(Error::ParsingError);
    }

    let mut interface = Interface::new(typename.as_str());
    for child in inner {
        parse_function(child, &mut interface)?;
    }
    parent_module.add_interface(&interface);
    Ok(())
}

fn parse_struct(pair: Pair<Rule>, parent_module: &mut Module) -> Result<(), Error> {
    let mut inner = pair.into_inner();
    let typename = inner.next().unwrap();
    if typename.as_rule() != Rule::typename {
        return Err(Error::ParsingError);
    }

    let mut structure = Struct::new(typename.as_str());
    for child in inner {
        parse_struct_line(child, &mut structure)?;
    }
    parent_module.add_struct(&structure);

    Ok(())
}

fn parse_enum(pair: Pair<Rule>, parent_module: &mut Module) -> Result<(), Error> {
    let mut inner = pair.into_inner();
    let typename = inner.next().unwrap();
    if typename.as_rule() != Rule::typename {
        return Err(Error::ParsingError);
    }

    let mut enumeration = Enum::new(typename.as_str());
    for child in inner {
        parse_enum_line(child, &mut enumeration)?;
    }
    parent_module.add_enum(&enumeration);
    Ok(())
}

fn parse_module(pair: Pair<Rule>, parent_module: &mut Module) -> Result<(), Error> {
    let mut inner = pair.into_inner();
    let typename = inner.next().unwrap();
    if typename.as_rule() != Rule::typename {
        return Err(Error::ParsingError);
    }

    let sub_module = parent_module.add_sub_module(typename.as_str())?;

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