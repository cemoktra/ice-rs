use regex::Regex;

#[derive(Clone, Debug)]
pub enum IceType {
    VoidType,
    BoolType,
    ByteType,
    ShortType,
    IntType,
    LongType,
    FloatType,
    DoubleType,
    StringType,
    SequenceType(Box<IceType>),
    DictType(Box<IceType>, Box<IceType>),
    CustomType(String)
}

impl IceType {
    pub fn from(text: &str) -> Result<IceType, Box<dyn std::error::Error>> {
        let type_re = Regex::new(
            r#"(?x)
            (void) |
            (bool) |
            (byte) |
            (short) |
            (int) |
            (long) |
            (float) |
            (double) |
            (string) |
            (sequence)<(.+)> |
            (dictionary)<(.+),\s*(.+)> |
            "#
        )?; 

        let captures = type_re.captures(text).map(|captures| {
            captures
                .iter() // All the captured groups
                .skip(1) // Skipping the complete match
                .flat_map(|c| c) // Ignoring all empty optional matches
                .map(|c| c.as_str()) // Grab the original strings
                .collect::<Vec<_>>() // Create a vector
        });

        match captures.as_ref().map(|c| c.as_slice()) {
            Some(["void"]) => Ok(IceType::VoidType),
            Some(["bool"]) => Ok(IceType::BoolType),
            Some(["byte"]) => Ok(IceType::ByteType),
            Some(["short"]) => Ok(IceType::ShortType),
            Some(["int"]) => Ok(IceType::IntType),
            Some(["long"]) => Ok(IceType::LongType),
            Some(["float"]) => Ok(IceType::FloatType),
            Some(["double"]) => Ok(IceType::DoubleType),
            Some(["string"]) => Ok(IceType::StringType),
            Some(["sequence", x]) => {
                Ok(IceType::SequenceType(Box::new(IceType::from(x)?)))
            },
            Some(["dictionary", x, y]) => {
                Ok(IceType::DictType(Box::new(IceType::from(x)?), Box::new(IceType::from(y)?)))
            },
            _ => Ok(IceType::CustomType(text.to_string()))
        }

        // match text {
        //     "void" => Ok(IceType::VoidType),
        //     "bool" => Ok(IceType::BoolType),
        //     "byte" => Ok(IceType::ByteType),
        //     "short" => Ok(IceType::ShortType),
        //     "int" => Ok(IceType::IntType),
        //     "long" => Ok(IceType::LongType),
        //     "float" => Ok(IceType::FloatType),
        //     "double" => Ok(IceType::DoubleType),
        //     "string" => Ok(IceType::StringType),
        //     // TODO: add generic type parsing
        //     // "sequence" => Ok(IceType::SequenceType()),
        //     // "dictionary" => Ok(IceType::DictType()),
        //     _ => Ok(IceType::CustomType(text.to_string()))
        // }
    }

    pub fn rust_type(&self) -> String {
        match self {
            IceType::VoidType => String::from("()"),
            IceType::BoolType => String::from("bool"),
            IceType::ByteType => String::from("u8"),
            IceType::ShortType => String::from("i16"),
            IceType::IntType => String::from("i32"),
            IceType::LongType => String::from("i64"),
            IceType::FloatType => String::from("f32"),
            IceType::DoubleType => String::from("f64"),
            IceType::StringType => String::from("String"),
            IceType::SequenceType(type_name) => format!("Vec<{}>", type_name.rust_type()),
            IceType::DictType(key_type, value_type) => format!("HashMap<{}, {}>", key_type.rust_type(), value_type.rust_type()),
            IceType::CustomType(type_name) => format!("{}", type_name),
        }
    }

    pub fn as_ref(&self) -> bool {
        match self {
            IceType::StringType |
            IceType::SequenceType(_) |
            IceType::DictType(_, _) |
            IceType::CustomType(_) => true,
            _ => false
        }
    }
}