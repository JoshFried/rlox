use std::fmt::{Display, Formatter};
use std::str;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    SingleCharacters(SingleCharacter),
    SingleOrDoubles(SingleOrDouble),
    Keywords(Keyword),
    Identifier,
}

impl TokenType {
    pub fn build_string(&self) -> String {
        match self {
            TokenType::SingleCharacters(token) => token.build_string(),
            TokenType::SingleOrDoubles(token) => token.build_string(),
            TokenType::Keywords(token) => token.build_string(),
            TokenType::Identifier => "identifier".to_string(),
        }
    }
}

impl TokenType {
    pub fn is_slash(&self) -> bool {
        matches!(self, TokenType::SingleCharacters(SingleCharacter::Slash))
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::SingleCharacters(token) => write!(f, "{}", token),
            TokenType::SingleOrDoubles(token) => write!(f, "{}", token),
            TokenType::Keywords(token) => write!(f, "{}", token),
            TokenType::Identifier => write!(f, "identifier"),
        }
    }
}

impl FromStr for TokenType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(token_type) = SingleCharacter::from_str(s) {
            return Ok(TokenType::SingleCharacters(token_type));
        }

        if let Ok(token_type) = SingleOrDouble::from_str(s) {
            return Ok(TokenType::SingleOrDoubles(token_type));
        }

        if let Ok(token_type) = Keyword::from_str(s) {
            return Ok(TokenType::Keywords(token_type));
        }

        Err(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SingleCharacter {
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

impl SingleCharacter {
    fn build_string(&self) -> String {
        match self {
            SingleCharacter::LeftParen => "(".to_string(),
            SingleCharacter::RightParen => ")".to_string(),
            SingleCharacter::LeftBrace => "{".to_string(),
            SingleCharacter::RightBrace => "}".to_string(),
            SingleCharacter::Comma => ",".to_string(),
            SingleCharacter::Dot => ".".to_string(),
            SingleCharacter::Minus => "-".to_string(),
            SingleCharacter::Plus => "+".to_string(),
            SingleCharacter::Semicolon => ";".to_string(),
            SingleCharacter::Slash => "/".to_string(),
            SingleCharacter::Star => "*".to_string(),
        }
    }
}

impl FromStr for SingleCharacter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "(" => Ok(SingleCharacter::LeftParen),
            ")" => Ok(SingleCharacter::RightParen),
            "{" => Ok(SingleCharacter::LeftBrace),
            "}" => Ok(SingleCharacter::RightBrace),
            "," => Ok(SingleCharacter::Comma),
            "." => Ok(SingleCharacter::Dot),
            "-" => Ok(SingleCharacter::Minus),
            "+" => Ok(SingleCharacter::Plus),
            ";" => Ok(SingleCharacter::Semicolon),
            "/" => Ok(SingleCharacter::Slash),
            "*" => Ok(SingleCharacter::Star),
            _ => Err(()),
        }
    }
}

impl Display for SingleCharacter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SingleCharacter::LeftParen => write!(f, "("),
            SingleCharacter::RightParen => write!(f, ")"),
            SingleCharacter::LeftBrace => write!(f, r#"{{"#),
            SingleCharacter::RightBrace => write!(f, r#"}}"#),
            SingleCharacter::Comma => write!(f, ","),
            SingleCharacter::Dot => write!(f, "."),
            SingleCharacter::Minus => write!(f, "-"),
            SingleCharacter::Plus => write!(f, "+"),
            SingleCharacter::Semicolon => write!(f, ";"),
            SingleCharacter::Slash => write!(f, "/"),
            SingleCharacter::Star => write!(f, "*"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SingleOrDouble {
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

impl SingleOrDouble {
    pub fn build_string(&self) -> String {
        match self {
            SingleOrDouble::Bang => "!".to_string(),
            SingleOrDouble::BangEqual => "!=".to_string(),
            SingleOrDouble::Equal => "=".to_string(),
            SingleOrDouble::EqualEqual => "==".to_string(),
            SingleOrDouble::Greater => ">".to_string(),
            SingleOrDouble::GreaterEqual => ">=".to_string(),
            SingleOrDouble::Less => "<".to_string(),
            SingleOrDouble::LessEqual => "<=".to_string(),
        }
    }
}

impl FromStr for SingleOrDouble {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "!" => Ok(SingleOrDouble::Bang),
            "!=" => Ok(SingleOrDouble::BangEqual),
            "=" => Ok(SingleOrDouble::Equal),
            "==" => Ok(SingleOrDouble::EqualEqual),
            ">" => Ok(SingleOrDouble::Greater),
            ">=" => Ok(SingleOrDouble::GreaterEqual),
            "<" => Ok(SingleOrDouble::Less),
            "<=" => Ok(SingleOrDouble::LessEqual),
            _ => Err(()),
        }
    }
}

impl Display for SingleOrDouble {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SingleOrDouble::Bang => write!(f, "!"),
            SingleOrDouble::BangEqual => write!(f, "!="),
            SingleOrDouble::Equal => write!(f, "="),
            SingleOrDouble::EqualEqual => write!(f, "=="),
            SingleOrDouble::Greater => write!(f, ">"),
            SingleOrDouble::GreaterEqual => write!(f, ">="),
            SingleOrDouble::Less => write!(f, "<"),
            SingleOrDouble::LessEqual => write!(f, "<="),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Literal<'literal> {
    String(&'literal str),
    Number(NumberType),
    Bool(bool),
    Nil,
}

impl<'literal> Literal<'literal> {
    pub fn build_string(&self) -> String {
        match self {
            Literal::String(str) => str.to_string(),
            Literal::Number(num) => num.build_string(),
            Literal::Bool(b) => b.to_string(),
            Literal::Nil => "nil".to_string(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NumberType {
    Integer(i64),
    Float(f64),
}

impl Display for NumberType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NumberType::Float(float) => write!(f, "{}", float),
            NumberType::Integer(int) => write!(f, "{}", int),
        }
    }
}

impl NumberType {
    pub fn build_string(&self) -> String {
        match self {
            NumberType::Integer(int) => int.to_string(),
            NumberType::Float(float) => float.to_string(),
        }
    }
}

impl FromStr for NumberType {
    type Err = ();

    // todo: improve error handling
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('.') {
            return Ok(NumberType::Float(s.parse::<f64>().expect("not a float")));
        }

        Ok(NumberType::Integer(s.parse::<i64>().expect("not an int")))
    }
}

impl<'literal> Display for Literal<'literal> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::String(s) => write!(f, "{}", s),
            Literal::Number(num) => write!(f, "{}", num),
            Literal::Nil => write!(f, "nil"),
            Literal::Bool(b) => write!(f, "{}", b),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Keyword {
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
    String,
    Integer,
    Float,
    Bool,
}

impl Keyword {
    pub fn build_string(&self) -> String {
        match self {
            Keyword::And => "and".to_string(),
            Keyword::Class => "class".to_string(),
            Keyword::Else => "else".to_string(),
            Keyword::False => "false".to_string(),
            Keyword::Fun => "fun".to_string(),
            Keyword::For => "for".to_string(),
            Keyword::If => "if".to_string(),
            Keyword::Nil => "nil".to_string(),
            Keyword::Or => "or".to_string(),
            Keyword::Print => "print".to_string(),
            Keyword::Return => "return".to_string(),
            Keyword::Super => "super".to_string(),
            Keyword::This => "this".to_string(),
            Keyword::True => "true".to_string(),
            Keyword::Var => "var".to_string(),
            Keyword::While => "while".to_string(),
            Keyword::Eof => "eof".to_string(),
            Keyword::String => "string".to_string(),
            Keyword::Integer => "integer".to_string(),
            Keyword::Float => "float".to_string(),
            Keyword::Bool => "bool".to_string(),
        }
    }
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "and" => Ok(Keyword::And),
            "class" => Ok(Keyword::Class),
            "else" => Ok(Keyword::Else),
            "false" => Ok(Keyword::False),
            "fun" => Ok(Keyword::Fun),
            "for" => Ok(Keyword::For),
            "if" => Ok(Keyword::If),
            "nil" => Ok(Keyword::Nil),
            "or" => Ok(Keyword::Or),
            "print" => Ok(Keyword::Print),
            "return" => Ok(Keyword::Return),
            "super" => Ok(Keyword::Super),
            "this" => Ok(Keyword::This),
            "true" => Ok(Keyword::True),
            "var" => Ok(Keyword::Var),
            "while" => Ok(Keyword::While),
            "eof" => Ok(Keyword::Eof),
            _ => Err(()),
        }
    }
}

impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword::And => write!(f, "and"),
            Keyword::Class => write!(f, "class"),
            Keyword::Else => write!(f, "else"),
            Keyword::False => write!(f, "false"),
            Keyword::Fun => write!(f, "fun"),
            Keyword::For => write!(f, "for"),
            Keyword::If => write!(f, "if"),
            Keyword::Nil => write!(f, "nil"),
            Keyword::Or => write!(f, "or"),
            Keyword::Print => write!(f, "print"),
            Keyword::Return => write!(f, "return"),
            Keyword::Super => write!(f, "super"),
            Keyword::This => write!(f, "this"),
            Keyword::True => write!(f, "true"),
            Keyword::Var => write!(f, "var"),
            Keyword::While => write!(f, "while"),
            Keyword::Eof => write!(f, "eof"),
            Keyword::String => write!(f, "string"),
            Keyword::Integer => write!(f, "inter"),
            Keyword::Float => write!(f, "float"),
            Keyword::Bool => write!(f, "bool"),
        }
    }
}
