use crate::expr::{Binary, Expr, Literal, Unary};
use crate::scanner::token::Token;
use crate::scanner::token_type::Literal::False;
use crate::scanner::token_type::SingleOrDouble::{Greater, GreaterEqual, Less, LessEqual};
use crate::scanner::token_type::{Keyword, SingleCharacter, SingleOrDouble, TokenType};
use std::cell::RefCell;
use std::rc::Rc;
use std::slice::SliceIndex;

pub struct Parser<'parser> {
    tokens: Rc<RefCell<Vec<Token<'parser>>>>,
    current: Rc<RefCell<usize>>,
}

impl<'parser> Parser<'parser> {
    pub fn new(tokens: Vec<Token<'parser>>) -> Self {
        Self {
            tokens: Rc::new(RefCell::new(tokens)),
            current: Rc::new(RefCell::new(0)),
        }
    }

    pub fn expression(&self) -> Expr {
        self.equality()
    }
    fn equality(&self) -> Expr {
        let mut expr = self.comparison();

        while self.does_match(vec![
            TokenType::SingleOrDoubles(SingleOrDouble::BangEqual),
            TokenType::SingleOrDoubles(SingleOrDouble::EqualEqual),
        ]) {
            let operator = self.previous();
            let right = self.comparison();

            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        expr
    }

    fn comparison(&self) -> Expr {
        let mut expr = self.term();

        while self.does_match(vec![
            TokenType::SingleOrDoubles(Greater),
            TokenType::SingleOrDoubles(GreaterEqual),
            TokenType::SingleOrDoubles(Less),
            TokenType::SingleOrDoubles(LessEqual),
        ]) {
            let operator = self.previous();

            let right = self.term();

            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        expr
    }

    fn does_match(&self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();

                return true;
            }
        }

        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_end() {
            false
        } else {
            token_type == *self.peek().get_type()
        }
    }

    fn advance(&self) -> Token {
        if !self.is_end() {
            let new_val = self.current.borrow().to_owned() + 1;
            self.current.replace(new_val);
        }

        self.previous()
    }

    fn previous(&self) -> Token {
        *self.tokens.borrow().get(0).unwrap()
    }

    fn is_end(&self) -> bool {
        *self.peek().get_type() == TokenType::Keywords(Keyword::Eof)
    }

    fn peek(&self) -> Token {
        self.tokens
            .borrow()
            .get(*self.current.borrow())
            .unwrap()
            .to_owned()
    }
    fn term(&self) -> Expr {
        let mut expr = self.factor();

        while self.does_match(vec![
            TokenType::SingleCharacters(SingleCharacter::Minus),
            TokenType::SingleCharacters(SingleCharacter::Plus),
        ]) {
            let operator = self.previous();
            let right = self.factor();

            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        expr
    }

    fn factor(&self) -> Expr {
        let mut expr = self.unary();

        while self.does_match(vec![
            TokenType::SingleCharacters(SingleCharacter::Slash),
            TokenType::SingleCharacters(SingleCharacter::Star),
        ]) {
            let operator = self.previous();
            let right = self.unary();

            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        expr
    }
    fn unary(&self) -> Expr {
        if self.does_match(vec![
            TokenType::SingleCharacters(SingleCharacter::Minus),
            TokenType::SingleCharacters(SingleCharacter::Plus),
        ]) {
            let operator = self.previous();

            return Expr::Unary(Unary {
                operator,
                right: Box::new(self.unary()),
            });
        }

        self.primary()
    }
    fn primary(&self) -> Expr {
        if self.does_match(vec![TokenType::Keywords(Keyword::False)]) {
            return Expr::Literal(Literal { value: False });
        }

        if self.does_match(vec![TokenType::Keywords(Keyword::False)]) {
            return Expr::Literal(Literal { value: False });
        }

        panic!("")
    }
}
