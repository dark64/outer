use crate::ast::node::Node;
use core::fmt;
use std::convert::TryFrom;

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
    Function(Vec<Type>, Option<Box<Type>>),
}

pub type TypeNode = Node<Type>;

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Int(width) => write!(f, "i{}", width),
            Type::UInt(width) => write!(f, "u{}", width),
            Type::Boolean => write!(f, "bool"),
            Type::String => write!(f, "string"),
            Type::Function(ref params, ref output) => {
                let params: Vec<String> = params.iter().map(|ty| format!("{}", ty)).collect();
                write!(f, "({})", params.join(", "))?;
                match output {
                    Some(ty) => write!(f, ": {}", ty),
                    _ => Ok(()),
                }
            }
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
            _ => {
                if value.starts_with("(") {
                    let index = value
                        .find(")")
                        .ok_or(TypeError::InvalidType(value.clone()))?;

                    let types: Result<Vec<_>, _> = value[1..index]
                        .split(",")
                        .filter(|s| !s.is_empty())
                        .map(|ty| Type::try_from(String::from(ty.trim())))
                        .collect();

                    if let Some(':') = value.chars().nth(index + 1) {
                        let output = String::from(value[index + 2..value.len()].trim());
                        let output = Type::try_from(output)?;
                        Ok(Type::Function(types?, Some(Box::new(output))))
                    } else {
                        Ok(Type::Function(types?, None))
                    }
                } else {
                    Err(TypeError::InvalidType(value))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_types() {
        let types: Result<Vec<_>, _> = vec![
            "i32",
            "i64",
            "u32",
            "u64",
            "bool",
            "string",
            "(i32, i32): i32",
            "(): bool",
            "()",
        ]
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
            Type::Function(
                vec![Type::Int(32), Type::Int(32)],
                Some(Box::new(Type::Int(32))),
            ),
            Type::Function(vec![], Some(Box::new(Type::Boolean))),
            Type::Function(vec![], None),
        ];

        for (i, _) in types.iter().enumerate() {
            assert_eq!(types[i], expected_types[i]);
        }
    }
}
