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
}