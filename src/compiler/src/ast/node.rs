use crate::position::Position;
use core::fmt;

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub start: Position,
    pub end: Position,
    pub value: T,
}

impl<T: fmt::Debug> Node<T> {
    pub fn from(value: T) -> Self {
        Self {
            start: Default::default(),
            end: Default::default(),
            value,
        }
    }
    pub fn set_start(mut self, start: Position) -> Self {
        self.start = start;
        self
    }
    pub fn set_end(mut self, end: Position) -> Self {
        self.end = end;
        self
    }
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T: fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
