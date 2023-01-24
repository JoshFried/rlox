pub mod token_type;

use crate::token::token_type::TokenType;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub struct Token<'token> {
    token_type: TokenType<'token>,
    lexeme: &'token str,
    literal: Option<&'token str>,
    line: usize,
}

impl<'token> Display for Token<'token> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.line)
    }
}

impl<'token> Token<'token> {
    pub fn new(
        token_type: TokenType<'token>,
        lexeme: &'token str,
        literal: Option<&'token str>,
        line: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}
