use crate::ast::node::Node;
use core::fmt;
use std::convert::TryFrom;
use std::fmt::Formatter;

#[derive(Debug, Clone)]
pub enum TypeError {
    InvalidType(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int(usize),
    UInt(usize),
    Boolean,
    String,
}

pub type TypeNode = Node<Type>;

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

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TypeError::InvalidType(s) => write!(f, "Invalid type `{}`", s),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_types() {
        let types: Result<Vec<_>, _> = vec!["i32", "i64", "u32", "u64", "bool", "string"]
            .iter()
            .map(|ty| Type::try_from(String::from(*ty)))
            .collect();

        let types = types.unwrap();
        let expected_types: Vec<Type> = vec![
            Type::Int(32),
            Type::Int(64),
            Type::UInt(32),
            Type::UInt(64),
            Type::Boolean,
            Type::String,
        ];

        for (i, _) in types.iter().enumerate() {
            assert_eq!(types[i], expected_types[i]);
        }
    }
}
