use crate::Token;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while let Some(token) = self.scanToken() {
            self.tokens.push(token);
        }

        &self.tokens
    }

    fn is_end(&self) -> bool {
        self.source.len() <= self.current
    }
    fn scanToken(&self) -> Option<Token> {
        if self.is_end() {
            return None;
        }

        let token = self.advance();

        match token {
            Token() => {}
        }
    }
    fn advance(&self) -> Token {
        todo!()
    }
}
