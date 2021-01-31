use crate::lexer::Lexer;
use crate::position::Position;

pub enum ParserErrorKind {
    UnexpectedToken,
}

pub struct ParserError {
    kind: ParserErrorKind,
    message: String,
    position: Option<Position>,
}

pub struct Parser<'ast> {
    lexer: Lexer<'ast>,
}

impl<'ast> Parser<'ast> {
    pub fn new(lexer: Lexer<'ast>) -> Self {
        Self { lexer }
    }

    pub fn parse_to_ast(&self) -> Result<(), ParserError> {
        unimplemented!()
    }
}
