use crate::position::Position;
use core::fmt;

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub start: Position,
    pub end: Position,
    pub value: T,
}

impl<T: fmt::Display> Node<T> {
    pub fn from(value: T, start: Position, end: Position) -> Self {
        Self { value, start, end }
    }
}

impl<T: fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
