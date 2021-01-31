pub mod tokens;

use crate::lexer::tokens::{Token, TokenType};
use crate::position::Position;
use std::iter::Peekable;
use std::str::Chars;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            input: code.chars().peekable(),
            line: 0,
            col: 0,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        if let Some(c) = self.input.next() {
            if self.is_linebreak(&c) {
                self.line += 1;
                self.col = 0;
            } else {
                self.col += 1;
            }
            Some(c)
        } else {
            None
        }
    }

    fn peek_char(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.input.peek() {
            if c.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let mut value = String::with_capacity(10);
        let position = self.get_position();
        let c = self.next_char()?;
        value.push(c);

        let token_type = match c {
            '=' => {
                if let Some(&'=') = self.peek_char() {
                    value.push(self.next_char().unwrap());
                    Some(TokenType::Eq)
                } else {
                    Some(TokenType::Assign)
                }
            }
            '+' => {
                if let Some(&'+') = self.peek_char() {
                    value.push(self.next_char().unwrap());
                    Some(TokenType::Increment)
                } else {
                    Some(TokenType::Plus)
                }
            }
            '-' => {
                if let Some(n) = self.peek_char() {
                    if *n == '-' {
                        value.push(self.next_char().unwrap());
                        Some(TokenType::Decrement)
                    } else if n.is_ascii_digit() {
                        self.read_number(&mut value);
                        Some(TokenType::IntLiteral)
                    } else {
                        Some(TokenType::Minus)
                    }
                } else {
                    Some(TokenType::Minus)
                }
            }
            '*' => {
                if let Some(&'*') = self.peek_char() {
                    value.push(self.next_char().unwrap());
                    Some(TokenType::Power)
                } else {
                    Some(TokenType::Asterisk)
                }
            }
            '/' => {
                if let Some(&'/') = self.peek_char() {
                    while let Some(n) = self.next_char() {
                        if self.is_linebreak(&n) {
                            self.col = 0;
                            break;
                        }
                    }
                    Some(TokenType::Comment)
                } else {
                    Some(TokenType::Slash)
                }
            }
            '%' => Some(TokenType::Percent),
            '?' => Some(TokenType::QuestionMark),
            '&' => {
                if let Some(&'&') = self.peek_char() {
                    value.push(self.next_char().unwrap());
                    Some(TokenType::And)
                } else {
                    Some(TokenType::BitAnd)
                }
            }
            '|' => {
                if let Some(&'|') = self.peek_char() {
                    value.push(self.next_char().unwrap());
                    Some(TokenType::Or)
                } else {
                    Some(TokenType::BitOr)
                }
            }
            '^' => Some(TokenType::BitXor),
            '<' => {
                let next_char = self.peek_char();
                if let Some(&'=') = next_char {
                    value.push(self.next_char().unwrap());
                    Some(TokenType::Lte)
                } else if let Some(&'<') = next_char {
                    value.push(self.next_char().unwrap());
                    Some(TokenType::BitLeftShift)
                } else {
                    Some(TokenType::Lt)
                }
            }
            '>' => {
                let next_char = self.peek_char();
                if let Some(&'=') = next_char {
                    value.push(self.next_char().unwrap());
                    Some(TokenType::Gte)
                } else if let Some(&'>') = next_char {
                    value.push(self.next_char().unwrap());
                    Some(TokenType::BitRightShift)
                } else {
                    Some(TokenType::Gt)
                }
            }
            '!' => {
                if let Some(&'=') = self.peek_char() {
                    value.push(self.next_char().unwrap());
                    Some(TokenType::Neq)
                } else {
                    Some(TokenType::Not)
                }
            }
            ',' => Some(TokenType::Comma),
            ';' => Some(TokenType::Semicolon),
            ':' => Some(TokenType::Colon),
            '(' => Some(TokenType::LParen),
            ')' => Some(TokenType::RParen),
            '{' => Some(TokenType::LBrace),
            '}' => Some(TokenType::RBrace),
            '[' => Some(TokenType::LBracket),
            ']' => Some(TokenType::RBracket),
            '"' => {
                value.pop(); // pop first quotation mark
                while let Some(n) = self.next_char() {
                    if n == '\\' && self.peek_char()? == &'"' {
                        // allow escaped quotation marks
                        value.push(n);
                        value.push(self.next_char()?);
                    } else if n == '"' {
                        break; // end of string literal
                    } else {
                        value.push(n);
                    }
                }
                Some(TokenType::StringLiteral)
            }
            _ => {
                if c == '_' || c.is_ascii_alphabetic() {
                    self.read_identifier(&mut value);
                    Some(
                        self.reserved_lookup(value.as_str())
                            .unwrap_or(TokenType::Identifier),
                    )
                } else if c.is_ascii_digit() {
                    self.read_number(&mut value);
                    Some(TokenType::IntLiteral)
                } else {
                    None
                }
            }
        }?;
        Some(Token::new(token_type, value, position))
    }

    fn get_position(&self) -> Position {
        Position {
            line: self.line,
            col: self.col,
        }
    }

    fn read_identifier(&mut self, value: &mut String) {
        while let Some(&c) = self.input.peek() {
            if c == '_' || c.is_ascii_alphabetic() || c.is_ascii_digit() {
                value.push(self.next_char().unwrap());
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self, value: &mut String) {
        while let Some(&c) = self.input.peek() {
            if c.is_ascii_digit() {
                value.push(self.next_char().unwrap());
            } else {
                break;
            }
        }
    }

    fn reserved_lookup(&self, id: &str) -> Option<TokenType> {
        match id {
            "i32" | "i64" | "u32" | "u64" | "bool" | "string" => Some(TokenType::VarType),
            "typedef" => Some(TokenType::Typedef),
            "func" => Some(TokenType::Function),
            "let" => Some(TokenType::Let),
            "auto" => Some(TokenType::Auto),
            "return" => Some(TokenType::Return),
            "if" => Some(TokenType::If),
            "else" => Some(TokenType::Else),
            "for" => Some(TokenType::For),
            "while" => Some(TokenType::While),
            "continue" => Some(TokenType::Continue),
            "break" => Some(TokenType::Break),
            "switch" => Some(TokenType::Switch),
            "case" => Some(TokenType::Case),
            "default" => Some(TokenType::Default),
            "true" => Some(TokenType::BooleanLiteral),
            "false" => Some(TokenType::BooleanLiteral),
            _ => None,
        }
    }

    fn is_linebreak(&self, c: &char) -> bool {
        *c == '\n'
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Token> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let lexer = Lexer::new(
            r#"
            // single line comment
            _var0 var1 123 -123 true false
            = + - * / % ** ++ -- == != < <= > >= ! ? & ^ | << >> && || , ; : ( ) [ ] { }
            "string literal" "escaped \"string literal\""
            typedef func let auto return if else for while continue break switch case default
            i32 u32 i64 u64 string bool
        "#,
        );

        let expected: Vec<TokenType> = vec![
            TokenType::Comment,        // // single line comment
            TokenType::Identifier,     // _var0
            TokenType::Identifier,     // var1
            TokenType::IntLiteral,     // 123
            TokenType::IntLiteral,     // -123
            TokenType::BooleanLiteral, // true
            TokenType::BooleanLiteral, // false
            TokenType::Assign,         // =
            TokenType::Plus,           // +
            TokenType::Minus,          // -
            TokenType::Asterisk,       // *
            TokenType::Slash,          // /
            TokenType::Percent,        // %
            TokenType::Power,          // **
            TokenType::Increment,      // ++
            TokenType::Decrement,      // --
            TokenType::Eq,             // ==
            TokenType::Neq,            // !=
            TokenType::Lt,             // <
            TokenType::Lte,            // <=
            TokenType::Gt,             // >
            TokenType::Gte,            // >=
            TokenType::Not,            // !
            TokenType::QuestionMark,   // ?
            TokenType::BitAnd,         // &
            TokenType::BitXor,         // ^
            TokenType::BitOr,          // |
            TokenType::BitLeftShift,   // <<
            TokenType::BitRightShift,  // >>
            TokenType::And,            // &&
            TokenType::Or,             // ||
            TokenType::Comma,          // ,
            TokenType::Semicolon,      // ;
            TokenType::Colon,          // :
            TokenType::LParen,         // (
            TokenType::RParen,         // )
            TokenType::LBracket,       // [
            TokenType::RBracket,       // ]
            TokenType::LBrace,         // {
            TokenType::RBrace,         // }
            TokenType::StringLiteral,  // "string literal"
            TokenType::StringLiteral,  // "escaped \"string literal\""
            TokenType::Typedef,        // typedef
            TokenType::Function,       // func
            TokenType::Let,            // let
            TokenType::Auto,           // auto
            TokenType::Return,         // return
            TokenType::If,             // if
            TokenType::Else,           // else
            TokenType::For,            // for
            TokenType::While,          // while
            TokenType::Continue,       // continue
            TokenType::Break,          // break
            TokenType::Switch,         // switch
            TokenType::Case,           // case
            TokenType::Default,        // default
            TokenType::VarType,        // i32
            TokenType::VarType,        // u32
            TokenType::VarType,        // i64
            TokenType::VarType,        // u64
            TokenType::VarType,        // string
            TokenType::VarType,        // bool
        ];

        let tokens: Vec<(Token, TokenType)> = lexer.into_iter().zip(expected).collect();

        for (token, token_type) in tokens {
            assert_eq!(token.ty, token_type);
        }
    }
}
