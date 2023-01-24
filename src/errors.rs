use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct Error(#[from] ErrorType);

#[derive(Error, Debug)]
pub enum ErrorType {
    #[error("Parsing error `{0}`")]
    Parse(String),
    #[error("Interpreter error")]
    Interpreter(String),
    #[error("Token error")]
    Token(String),
    #[error("io")]
    Io(#[from] io::Error),
}
