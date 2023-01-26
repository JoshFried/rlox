use crate::errors::Error;
pub use std::str;

pub mod pretty_printer;
pub mod scanner;
pub mod token;
pub mod token_type;

const NEW_LINE: u8 = b'\n';
const SLASH: u8 = b'/';
const WHITE_SPACE: u8 = b' ';
const CARRIAGE_RETURN: u8 = b'\r';
const TAB: u8 = b'\t';
const QUOTE: u8 = b'"';
const PERIOD: u8 = b'.';

type Result<T> = std::result::Result<T, Error>;
