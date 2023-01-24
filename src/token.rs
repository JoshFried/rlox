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

#[derive(Display)]
enum TokenType {
    SingleCharacter(SingleCharacterToken),
    SingleOrDouble(SingleOrDoubleCharacterToken),
    Literals(Literals),
    Keywords(Keywords),
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::SingleCharacter(token) => write!(f, token),
            TokenType::SingleOrDouble(token) => write!(f, token),
            TokenType::Literals(token) => write!(f, token),
            TokenType::Keywords(token) => write!(f, token),
        }
    }
}

enum SingleCharacterToken {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
}

impl Display for SingleCharacterToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SingleCharacterToken::LeftParen => write!(f, "("),
            SingleCharacterToken::RightParen => write!(f, ")"),
            SingleCharacterToken::LeftBrace => write!(f, '{'),
            SingleCharacterToken::RightBrace => write!(f, '}'),
            SingleCharacterToken::Comma => write!(f, ","),
            SingleCharacterToken::Dot => write!(f, "."),
            SingleCharacterToken::Minus => write!(f, "-"),
            SingleCharacterToken::Plus => write!(f, "+"),
            SingleCharacterToken::Semicolon => write!(f, ";"),
            SingleCharacterToken::Slash => write!(f, "/"),
            SingleCharacterToken::Star => write!(f, "*"),
        }
    }
}

enum SingleOrDoubleCharacterToken {
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

impl Display for SingleOrDoubleCharacterToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SingleOrDoubleCharacterToken::Bang => write!(f, "!"),
            SingleOrDoubleCharacterToken::BangEqual => write!(f, "!="),
            SingleOrDoubleCharacterToken::Equal => write!(f, "="),
            SingleOrDoubleCharacterToken::EqualEqual => write!(f, "=="),
            SingleOrDoubleCharacterToken::Greater => write!(f, ">"),
            SingleOrDoubleCharacterToken::GreaterEqual => write!(f, ">="),
            SingleOrDoubleCharacterToken::Less => write!(f, "<"),
            SingleOrDoubleCharacterToken::LessEqual => write!(f, "<="),
        }
    }
}

enum Literals {
    Identifier,
    String,
    Number,
}

impl Display for Literals {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literals::Identifier => write!(f, "identifier"),
            Literals::String => write!(f, "string"),
            Literals::Number => write!(f, "number"),
        }
    }
}

enum Keywords {
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

impl Display for Keywords {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match Self {
            Keywords::And => write!(f, "and"),
            Keywords::Class => write!(f, "class"),
            Keywords::Else => write!(f, "else"),
            Keywords::False => write!(f, "false"),
            Keywords::Fun => write!(f, "fun"),
            Keywords::For => write!(f, "for"),
            Keywords::If => write!(f, "if"),
            Keywords::Nil => write!(f, "nil"),
            Keywords::Or => write!(f, "or"),
            Keywords::Print => write!(f, "print"),
            Keywords::Return => write!(f, "return"),
            Keywords::Super => write!(f, "super"),
            Keywords::This => write!(f, "this"),
            Keywords::True => write!(f, "true"),
            Keywords::Var => write!(f, "var"),
            Keywords::While => write!(f, "while"),
            Keywords::Eof => write!(f, "eof"),
        }
    }
}
