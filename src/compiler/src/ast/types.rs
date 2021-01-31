use core::fmt;
use std::convert::TryFrom;

pub enum TypeError {
    InvalidType(String),
}

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Int(usize),
    UInt(usize),
    Boolean,
    String,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Int(width) => write!(f, "i{}", width),
            Type::UInt(width) => write!(f, "u{}", width),
            Type::Boolean => write!(f, "bool"),
            Type::String => write!(f, "string"),
        }
    }
}

impl TryFrom<String> for Type {
    type Error = TypeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "i32" => Ok(Type::Int(32)),
            "i64" => Ok(Type::Int(64)),
            "u32" => Ok(Type::UInt(32)),
            "u64" => Ok(Type::UInt(64)),
            "bool" => Ok(Type::Boolean),
            "string" => Ok(Type::String),
            _ => Err(TypeError::InvalidType(value)),
        }
    }
}
