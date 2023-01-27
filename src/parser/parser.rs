use crate::errors::ErrorType;
use crate::expr::{Binary, Expr, GroupingExpr, LiteralExpr, Unary};
use crate::scanner::token::Token;
use crate::scanner::token_type::{Keyword, Literal, SingleCharacter, SingleOrDouble, TokenType};
use crate::Result;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Parser<'parser> {
    tokens: Rc<RefCell<Vec<Token<'parser>>>>,
    current: Rc<RefCell<usize>>,
}

const EQUALITY_MATCH: [TokenType; 2] = [
    TokenType::SingleOrDoubles(SingleOrDouble::BangEqual),
    TokenType::SingleOrDoubles(SingleOrDouble::EqualEqual),
];

const FACTOR_MATCH: [TokenType; 2] = [
    TokenType::SingleCharacters(SingleCharacter::Slash),
    TokenType::SingleCharacters(SingleCharacter::Star),
];

const ADD_AND_SUBTRACT_MATCH: [TokenType; 2] = [
    TokenType::SingleCharacters(SingleCharacter::Minus),
    TokenType::SingleCharacters(SingleCharacter::Plus),
];

const INEQUALITY_MATCH: [TokenType; 4] = [
    TokenType::SingleOrDoubles(SingleOrDouble::Greater),
    TokenType::SingleOrDoubles(SingleOrDouble::GreaterEqual),
    TokenType::SingleOrDoubles(SingleOrDouble::Less),
    TokenType::SingleOrDoubles(SingleOrDouble::LessEqual),
];

impl<'parser> Parser<'parser> {
    pub fn new(tokens: Vec<Token<'parser>>) -> Self {
        Self {
            tokens: Rc::new(RefCell::new(tokens)),
            current: Rc::new(RefCell::new(0)),
        }
    }

    pub fn expression(&self) -> Result<Expr> {
        self.equality()
    }

    fn equality(&self) -> Result<Expr> {
        let mut expr = self.comparison()?;

        while self.does_match(Vec::from(EQUALITY_MATCH)) {
            let operator = self.previous();
            let right = self.comparison()?;

            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn comparison(&self) -> Result<Expr> {
        let mut expr = self.term()?;

        while self.does_match(Vec::from(INEQUALITY_MATCH)) {
            let operator = self.previous();
            let right = self.term()?;

            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            })
        }

        Ok(expr)
    }

    fn term(&self) -> Result<Expr> {
        let mut expr = self.factor()?;
        while self.does_match(Vec::from(ADD_AND_SUBTRACT_MATCH)) {
            let operator = self.previous();
            let right = self.factor()?;

            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn factor(&self) -> Result<Expr> {
        let mut expr = self.unary()?;

        while self.does_match(Vec::from(FACTOR_MATCH)) {
            let operator = self.previous();
            let right = self.unary()?;

            expr = Expr::Binary(Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
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

    fn unary(&self) -> Result<Expr> {
        if self.does_match(Vec::from(ADD_AND_SUBTRACT_MATCH)) {
            let operator = self.previous();

            return Ok(Expr::Unary(Unary {
                operator,
                right: Box::new(self.unary()?),
            }));
        }

        self.primary()
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_end() {
            false
        } else {
            token_type == *self.peek().get_type()
        }
    }

    fn primary(&self) -> Result<Expr> {
        if self.does_match(vec![TokenType::Keywords(Keyword::False)]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Literal::Bool(false),
            }));
        }

        if self.does_match(vec![TokenType::Keywords(Keyword::True)]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Literal::Bool(true),
            }));
        }

        if self.does_match(vec![TokenType::Keywords(Keyword::Nil)]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: Literal::Nil,
            }));
        }

        if self.does_match(vec![
            TokenType::Keywords(Keyword::String),
            TokenType::Keywords(Keyword::Integer),
            TokenType::Keywords(Keyword::Float),
        ]) {
            return Ok(Expr::Literal(LiteralExpr {
                value: self.previous().literal().expect("expected literal "),
            }));
        }

        if self.does_match(vec![TokenType::SingleCharacters(
            SingleCharacter::LeftParen,
        )]) {
            let expr = self.expression()?;
            self.consume(SingleCharacter::RightParen)?;

            return Ok(Expr::Grouping(GroupingExpr {
                expression: Box::new(expr),
            }));
        }

        Err(crate::errors::Error(ErrorType::Interpreter(String::from(
            "error matching token",
        ))))
    }

    fn advance(&self) -> Result<Token> {
        if !self.is_end() {
            let new_val = self.current.borrow().to_owned() + 1;
            self.current.replace(new_val);
        }

        Ok(self.previous())
    }

    fn previous(&self) -> Token<'parser> {
        *self
            .tokens
            .borrow()
            .get(*(self.current.borrow()) - 1)
            .expect("unable to get previous")
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

    fn consume(&self, token_type: SingleCharacter) -> Result<()> {
        if self.check(TokenType::SingleCharacters(token_type)) {
            self.advance()?;

            return Ok(());
        }

        Err(crate::errors::Error(ErrorType::Interpreter(String::from(
            "error while consuming",
        ))))
    }
}
