use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct Error(#[from] ErrorType);

#[derive(Error, Debug)]
pub enum ErrorType {
    #[error("Parsing error `{0}`")]
    ParseError(String),
    #[error("Interpreter error")]
    InterpreterError(String),
    #[error("Token error")]
    TokenError(String),
    #[error("io")]
    IoError(#[from] io::Error),
}
