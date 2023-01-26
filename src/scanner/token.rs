use crate::scanner::token_type::TokenType;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
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

    // todo: i think theres a more idiomatic way to go about this
    pub fn build_string(&self) -> String {
        self.token_type.build_string()
    }
}
