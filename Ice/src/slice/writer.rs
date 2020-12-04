use crate::errors::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn write_to_bytes(file: &mut File, object_name: &str, lines: Vec<String>) -> Result<(), Error> {
    file.write_all(format!("impl ToBytes for {} {{\n", &object_name).as_bytes())?;
    file.write_all("    fn to_bytes(&self) -> Result<Vec<u8>, Error> {\n".as_bytes())?;
    file.write_all("        let mut bytes = Vec::new();\n".as_bytes())?;
    for line in lines {
        file.write_all(format!("        {}", line).as_bytes())?;
    }
    file.write_all("        Ok(bytes)\n".as_bytes())?;
    file.write_all("    }\n}\n\n".as_bytes())?;

    Ok(())
}

pub fn write_from_bytes(file: &mut File, object_name: &str, lines: Vec<String>) -> Result<(), Error> {
    file.write_all(format!("impl FromBytes for {} {{\n", &object_name).as_bytes())?;
    file.write_all("    fn from_bytes(bytes: &[u8], read_bytes: &mut i32) -> Result<Self, Error> {\n".as_bytes())?;
    file.write_all("        let mut read = 0;\n".as_bytes())?;
    file.write_all("        let obj = Self{\n".as_bytes())?;
    for line in lines {
        file.write_all(format!("            {}", line).as_bytes())?;
    }
    file.write_all("        };\n".as_bytes())?;
    file.write_all("        *read_bytes = *read_bytes + read;\n".as_bytes())?;
    file.write_all("        Ok(obj)\n".as_bytes())?;
    file.write_all("    }\n}\n\n".as_bytes())?;

    Ok(())
}

pub fn write_encapsulation(file: &mut File, object_name: &str) -> Result<(), Error> {
    file.write_all(format!("impl AsEncapsulation for {} {{\n", object_name).as_bytes())?;
    file.write_all("    fn as_encapsulation(&self) -> Result<Encapsulation, Error> {\n".as_bytes())?;
    file.write_all("        let bytes = self.to_bytes()?;\n".as_bytes())?;
    file.write_all("        Ok(Encapsulation {\n".as_bytes())?;
    file.write_all("            size: 6 + bytes.len() as i32,\n".as_bytes())?;
    file.write_all("            major: 1,\n".as_bytes())?;
    file.write_all("            minor: 1,\n".as_bytes())?;
    file.write_all("            data: bytes.to_vec(),\n".as_bytes())?;
    file.write_all("        })\n".as_bytes())?;
    file.write_all("    }\n}\n\n".as_bytes())?;

    file.write_all(format!("impl FromEncapsulation for {} {{\n", object_name).as_bytes())?;
    file.write_all("    type Output = Self;\n\n".as_bytes())?;
    file.write_all("    fn from_encapsulation(body: Encapsulation) -> Result<Self::Output, Error> {\n".as_bytes())?;
    file.write_all("        let mut read_bytes = 0;\n".as_bytes())?;
    file.write_all("        Self::from_bytes(&body.data, &mut read_bytes)\n".as_bytes())?;
    file.write_all("    }\n}\n\n".as_bytes())?;

    Ok(())
}