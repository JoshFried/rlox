use crate::errors::Error;
use crate::errors::ErrorType::ParseError;
use crate::token::token_type::Literal::String as StringLiteral;
use crate::token::token_type::TokenType;
use crate::token::Token;
use std::str;
use std::str::FromStr;

const NEW_LINE: u8 = b'\n';
const SLASH: u8 = b'/';
const WHITE_SPACE: u8 = b' ';
const CARRIAGE_RETURN: u8 = b'\r';
const TAB: u8 = b'\t';
const QUOTE: u8 = b'"';

type Result<T> = std::result::Result<T, Error>;

pub struct Scanner<'scanner> {
    source: &'scanner [u8],
    tokens: Vec<Token<'scanner>>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'scanner> Scanner<'scanner> {
    pub fn new(source: &'scanner [u8]) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        //TODO: theres gotta be a better way....
        while let Some(..) = self.scan_token() {}

        self.tokens.push(Token::new(
            TokenType::from_str("eof").unwrap(), // safe cause we know eof is a valid from_str
            "",
            None,
            self.line,
        ));

        &self.tokens
    }

    fn is_end(&self) -> bool {
        self.source.len() <= self.current
    }

    fn scan_token(&mut self) -> Option<()> {
        if self.is_end() {
            return None;
        }

        let token_as_u8 = vec![self.advance()];

        // TODO: will this work as expected? ... Make sure to write a test
        if is_skip(token_as_u8[0]) {
            return Some(());
        }

        let token_as_str: &str = str::from_utf8(&token_as_u8).unwrap(); //todo: fix

        if let Ok(token) = TokenType::from_str(token_as_str) {
            if token.is_slash() {
                if self.match_token(SLASH) {
                    // grab all the comments till End of Line
                    while self.peek() != NEW_LINE && !self.is_end() {
                        self.advance();
                    }
                }

                return Some(());
            }

            self.add_token(token);

            return Some(());
        }

        // TODO: is this safe to do?
        let token_as_u8 = [token_as_u8[0], self.advance()];
        let token_as_str: &str = str::from_utf8(&token_as_u8).unwrap(); //todo: fix

        if let Ok(token) = TokenType::from_str(token_as_str) {
            self.add_token(token);

            return Some(());
        }

        if token_as_u8[0] == QUOTE {
            self.handle_string_literal();
        }

        None
    }

    fn handle_string_literal(&mut self) -> Result<Option<()>> {
        while self.peek() != QUOTE && !self.is_end() {
            if self.peek() == NEW_LINE {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_end() {
            // TODO: add a logger
            return Err(Error::from(ParseError(String::from(
                "error attempting to parse string literal",
            )))); //todo: this should return Option<Result<()>> for this exact reason
        }

        self.advance();

        let result = str::from_utf8(&self.source[self.start + 1..self.current - 1]).unwrap();
        self.add_token(TokenType::Literals(StringLiteral(result)));

        Ok(Some(()))
    }

    fn add_token(&mut self, token: TokenType<'scanner>) {
        let token = self.build_token(token);
        self.tokens.push(token);
    }

    fn advance(&mut self) -> u8 {
        let token = self.source[self.current];
        self.current += 1;

        token
    }

    fn peek(&self) -> u8 {
        match self.is_end() {
            true => 0,
            false => self.source[self.current],
        }
    }
    fn match_token(&mut self, token: u8) -> bool {
        if self.is_end() {
            return false;
        }

        if self.peek() != token {
            return false;
        }

        self.current += 1;
        true
    }

    fn build_token(&self, token: TokenType<'scanner>) -> Token<'scanner> {
        let text = self.get_text();
        Token::new(token, text, None, self.line)
    }

    fn get_text(&self) -> &'scanner str {
        str::from_utf8(&self.source[self.start..self.current]).unwrap()
    }
}

fn is_skip(character: u8) -> bool {
    character == CARRIAGE_RETURN
        || character == WHITE_SPACE
        || character == NEW_LINE
        || character == TAB
}
