use crate::errors::Error;
use crate::errors::ErrorType::Parse;
use crate::scanner;
use crate::scanner::token::Token;
use crate::scanner::token_type::Literal::{Identifier, Number, String as StringLiteral};
use crate::scanner::token_type::{Keyword, NumberType, TokenType};
use crate::scanner::{CARRIAGE_RETURN, NEW_LINE, PERIOD, QUOTE, SLASH, TAB, WHITE_SPACE};
use std::str;
use std::str::FromStr;

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
            return match self.handle_string_literal() {
                Ok(..) => Some(()),
                Err(..) => None,
            };
        }

        if token_as_u8[0].is_ascii_digit() {
            return match self.handle_number_literal() {
                Ok(..) => Some(()),
                Err(..) => None,
            };
        }

        if token_as_u8[0].is_ascii_alphabetic() {
            match self.handle_identifier_literal() {
                Ok(..) => Some(()),
                Err(..) => None,
            };
        }

        None
    }

    fn handle_string_literal(&mut self) -> scanner::Result<Option<()>> {
        while self.peek() != QUOTE && !self.is_end() {
            if self.peek() == NEW_LINE {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_end() {
            // TODO: add a logger
            return Err(Error::from(Parse(String::from(
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

    fn handle_number_literal(&mut self) -> scanner::Result<Option<()>> {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == PERIOD && self.peek_next().is_ascii_digit() {
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let result = str::from_utf8(&self.source[self.start + 1..self.current - 1]).unwrap();
        let token = match result.parse::<i64>() {
            Ok(number) => TokenType::Literals(Number(NumberType::Integer(number))),
            Err(..) => {
                TokenType::Literals(Number(NumberType::Float(result.parse::<f64>().unwrap())))
            }
        };

        self.add_token(token);

        Ok(Some(()))
    }

    fn peek_next(&self) -> u8 {
        match self.current + 1 >= self.source.len() {
            true => b'\0',
            false => self.source[self.current + 1],
        }
    }

    fn handle_identifier_literal(&mut self) -> scanner::Result<Option<()>> {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        let word = self.get_text();

        match Keyword::from_str(word) {
            Ok(keyword) => self.add_token(TokenType::Keywords(keyword)),
            Err(..) => self.add_token(TokenType::Literals(Identifier)),
        }

        Ok(Some(()))
    }
}

fn is_skip(character: u8) -> bool {
    character == CARRIAGE_RETURN
        || character == WHITE_SPACE
        || character == NEW_LINE
        || character == TAB
}
