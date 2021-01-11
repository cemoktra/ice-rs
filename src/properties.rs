use pest::Parser;
use std::collections::BTreeMap;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;


#[derive(Parser)]
#[grammar = "iceconfig.pest"]
pub struct PropertyParser;


pub struct Properties {
    properties: BTreeMap<String, String>
}

impl Properties {
    pub fn new() -> Properties {
        Properties {
            properties: BTreeMap::new()
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        return self.properties.get(key)
    }

    pub fn has(&self, key: &str) -> bool {
        return self.properties.contains_key(key)
    }

    pub fn load(&mut self, config_file: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut content = String::new();
        let mut file = File::open(Path::new(&config_file))?;
        file.read_to_string(&mut content)?;
        let mut pairs = PropertyParser::parse(Rule::iceconfig, &content).unwrap();
        let mut key = "";

        let config = pairs.next().unwrap();
        if config.as_rule() == Rule::iceconfig {
            for pair in config.into_inner() {
                match pair.as_rule() {
                    Rule::property_key => {
                        key = pair.as_str();
                    }
                    Rule::property_value => {
                        self.properties.insert(String::from(key), String::from(pair.as_str()));
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }
}