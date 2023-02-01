use crate::scanner::token_type::{Literal, TokenType};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Token<'token> {
    token_type: TokenType,
    lexeme: &'token str,
    literal: Option<Literal<'token>>,
    line: usize,
}

impl<'token> Display for Token<'token> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.line)
    }
}

impl<'token> Token<'token> {
    pub fn new(
        token_type: TokenType,
        lexeme: &'token str,
        literal: Option<Literal<'token>>,
        line: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn get_type(&self) -> &TokenType {
        &self.token_type
    }

    // todo: i think theres a more idiomatic way to go about this
    pub fn build_string(&self) -> String {
        self.token_type.build_string()
    }

    pub fn literal(&self) -> Option<Literal<'token>> {
        self.literal
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn lexeme(&self) -> &'token str {
        self.lexeme
    }
}
