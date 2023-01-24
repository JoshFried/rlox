mod token_type;

use crate::token::token_type::TokenType;
use std::fmt::{Display, Formatter};

pub struct Token<'token, T>
where
    T: Display,
{
    token_type: TokenType,
    lexeme: &'token str,
    literal: T,
    line: usize,
}

impl<'token, T> Display for Token<'token, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.line)
    }
}

impl<'token, T> Token<'token, T> {
    pub fn new(token_type: TokenType, lexeme: &'token str, literal: T, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}
