use crate::errors::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn write(file: &mut File, line: &str, indent: usize) -> Result<(), Error> {
    file.write_all(format!("{:width$}{}", "", line, width=(4 * indent)).as_bytes())?;
    Ok(())
}

pub fn write_to_bytes(file: &mut File, object_name: &str, lines: Vec<String>) -> Result<(), Error> {
    write(file, &format!("impl ToBytes for {} {{\n", &object_name), 0)?;
    write(file, "fn to_bytes(&self) -> Result<Vec<u8>, Error> {\n", 1)?;
    write(file, "let mut bytes = Vec::new();\n", 2)?;
    for line in lines {
        write(file, &line, 2)?;
    }
    write(file, "Ok(bytes)\n", 2)?;
    write(file, "}\n}\n\n", 1)
}

pub fn write_from_bytes(file: &mut File, object_name: &str, lines: Vec<String>) -> Result<(), Error> {
    write(file, &format!("impl FromBytes for {} {{\n", &object_name), 0)?;
    write(file, "fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error> {\n", 1)?;
    write(file, "let mut read = 0;\n", 2)?;
    write(file, "let obj = Self{\n", 2)?;
    for line in lines {
        write(file, &line, 3)?;
    }
    write(file, "};\n", 2)?;
    write(file, "*read_bytes = *read_bytes + read;\n", 2)?;
    write(file, "Ok(obj)\n", 2)?;
    write(file, "}\n}\n\n", 1)
}
