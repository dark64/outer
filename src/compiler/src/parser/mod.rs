use crate::ast::{
    BlockStatement, BlockStatementNode, DefinitionType, ExpressionNode, Program, Statement,
    StatementNode, Variable, VariableNode,
};
use crate::lexer::tokens::{Token, TokenType};
use crate::lexer::Lexer;
use crate::position::Position;
use std::convert::TryInto;
use std::iter::Peekable;

#[derive(Debug, Clone)]
pub enum ParserErrorKind {
    UnexpectedToken,
    UnexpectedEOF,
    InvalidType,
}

#[derive(Debug, Clone)]
pub struct ParserError {
    kind: ParserErrorKind,
    message: String,
    position: Option<Position>,
}

impl ParserError {
    pub fn new(kind: ParserErrorKind, message: String) -> Self {
        Self {
            kind,
            message,
            position: None,
        }
    }
    pub fn set_position(mut self, position: Position) -> Self {
        self.position = Some(position);
        self
    }
    pub fn eof() -> Self {
        ParserError::new(
            ParserErrorKind::UnexpectedEOF,
            format!("Unexpected end of file (EOF)"),
        )
    }
}

pub struct Parser<'ast> {
    lexer: Peekable<Lexer<'ast>>,
}

impl<'ast> Parser<'ast> {
    pub fn new(lexer: Lexer<'ast>) -> Self {
        Self {
            lexer: lexer.into_iter().peekable(),
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, ParserError> {
        let mut statements: Vec<StatementNode> = vec![];
        while let Some(s) = self.consume_statement()? {
            statements.push(s);
        }
        Ok(Program(statements))
    }

    fn consume_statement(&mut self) -> Result<Option<StatementNode>, ParserError> {
        if self.peek_unchecked().is_none() {
            return Ok(None);
        }

        let token = self.consume_unchecked()?;
        let start = token.position.clone();

        let statement = match token.ty {
            TokenType::Let => {
                let var = self.consume_variable_definition()?;
                let statement = if self.peek(TokenType::Assign) {
                    self.consume(TokenType::Assign)?;
                    Statement::Definition(DefinitionType::Let, var, self.consume_expression()?)
                } else {
                    Statement::Declaration(var)
                };
                let semicolon = self.consume(TokenType::Semicolon)?;
                Ok(StatementNode::from(statement)
                    .set_start(start)
                    .set_end(semicolon.position))
            }
            TokenType::Auto => {
                let identifier = self.consume(TokenType::Identifier)?;
                self.consume(TokenType::Assign)?;
                let statement = Statement::Definition(
                    DefinitionType::Auto,
                    VariableNode::from(Variable::new(identifier.value, None)),
                    self.consume_expression()?,
                );
                let semicolon = self.consume(TokenType::Semicolon)?;
                Ok(StatementNode::from(statement)
                    .set_start(start)
                    .set_end(semicolon.position))
            }
            TokenType::Return => {
                let expr = self.consume_expression()?;
                let semicolon = self.consume(TokenType::Semicolon)?;
                Ok(StatementNode::from(Statement::Return(expr))
                    .set_start(start)
                    .set_end(semicolon.position))
            }
            TokenType::If => {
                let condition = self.consume_expression()?;
                let consequence = self.consume_block()?;
                if self.peek(TokenType::Else) {
                    self.consume(TokenType::Else)?;
                    let alternative = self.consume_block()?;
                    let statement =
                        Statement::Condition(condition, consequence, Some(alternative.clone()));
                    Ok(StatementNode::from(statement)
                        .set_start(start)
                        .set_end(alternative.end))
                } else {
                    let statement = Statement::Condition(condition, consequence.clone(), None);
                    Ok(StatementNode::from(statement)
                        .set_start(start)
                        .set_end(consequence.end))
                }
            }
            _ => Err(ParserError::eof()),
        }?;
        Ok(Some(statement))
    }

    fn consume_variable_definition(&mut self) -> Result<VariableNode, ParserError> {
        let identifier = self.consume(TokenType::Identifier)?;
        self.consume(TokenType::Colon)?;
        let type_token = self.consume(TokenType::VarType)?;
        let var_type = type_token.clone().value.try_into().map_err(|e| {
            ParserError::new(ParserErrorKind::InvalidType, format!("{}", e))
                .set_position(type_token.position)
        })?;
        Ok(VariableNode::from(Variable::new(
            identifier.value,
            Some(var_type),
        )))
    }

    fn consume_block(&mut self) -> Result<BlockStatementNode, ParserError> {
        let mut statements: Vec<StatementNode> = vec![];
        let left_brace = self.consume(TokenType::LBrace)?;

        while !self.peek(TokenType::RBrace) {
            match self.consume_statement()? {
                Some(s) => statements.push(s),
                None => break,
            }
        }

        let right_brace = self.consume(TokenType::RBrace)?;
        let block = BlockStatement(statements);

        Ok(BlockStatementNode::from(block)
            .set_start(left_brace.position)
            .set_end(right_brace.position))
    }

    fn consume_expression(&self) -> Result<ExpressionNode, ParserError> {
        unimplemented!()
    }

    fn consume_unchecked(&mut self) -> Result<Token, ParserError> {
        Ok(self.lexer.next().ok_or(ParserError::eof())?)
    }

    fn consume(&mut self, expected_type: TokenType) -> Result<Token, ParserError> {
        let token = self.lexer.next().ok_or(ParserError::eof())?;
        if token.ty == expected_type {
            Ok(token)
        } else {
            Err(ParserError::new(
                ParserErrorKind::UnexpectedToken,
                format!("Expected token {:?}, but got {:?}", expected_type, token.ty),
            )
            .set_position(token.position))
        }
    }

    fn peek_unchecked(&mut self) -> Option<&Token> {
        self.lexer.peek()
    }

    fn peek_next(&mut self, expected_type: TokenType) -> Option<&Token> {
        let token = self.lexer.peek()?;
        if token.ty == expected_type {
            Some(token)
        } else {
            None
        }
    }

    fn peek(&mut self, expected_type: TokenType) -> bool {
        if let Some(t) = self.lexer.peek() {
            t.ty == expected_type
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::types::Type;

    #[test]
    fn let_declaration() {
        let code = r#"let a: i32;"#;
        let lexer = Lexer::new(code);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();
        let statements = program.0;

        assert_eq!(
            statements,
            vec![StatementNode::from(Statement::Declaration(
                VariableNode::from(Variable::new(String::from("a"), Some(Type::Int(32))))
            ))]
        )
    }
}
