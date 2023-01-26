mod errors;
mod expr;
mod parser;
mod scanner;

use errors::Error;
use expr::{Binary, Expr, Literal, Unary};
use scanner::pretty_printer::PrettyPrinter;
use scanner::scanner::Scanner;
use scanner::token_type::NumberType::{Float, Integer};
use scanner::token_type::{SingleCharacter, TokenType};
use scanner::{token, token_type};
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::process::exit;
use std::{env, fs};
use token::Token;

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let expression = Expr::Binary(Binary {
        left: Box::new(Expr::Unary(Unary {
            operator: Token::new(
                TokenType::SingleCharacters(SingleCharacter::Minus),
                "-",
                None,
                1,
            ),
            right: Box::new(Expr::Literal(Literal {
                value: token_type::Literal::Number(Integer(123)),
            })),
        })),
        operator: Token::new(
            TokenType::SingleCharacters(SingleCharacter::Star),
            "*",
            None,
            1,
        ),
        right: Box::new(Expr::Grouping(expr::Grouping {
            expression: Box::new(Expr::Literal(Literal {
                value: token_type::Literal::Number(Float(45.67)),
            })),
        })),
    });

    let printer = PrettyPrinter::new();
    printer.print_expr(&expression);

    let x = match args.len() {
        2 => run_file(&args[1]),
        1 => run_prompt(),
        _ => panic!("fucked up"),
    }?;

    exit(x);
}

fn run_prompt() -> Result<i32> {
    loop {
        println!(">");
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("failed to read line");

        run(&line)?;
    }
}

fn run(line: &str) -> Result<()> {
    let mut scanner = Scanner::new(line.as_bytes());

    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }

    Ok(())
}

fn run_file(file_path: &str) -> Result<i32> {
    if !Path::new(file_path).exists() {
        return Ok(-1);
    }

    let buffer = read_source_file(file_path)?;

    let mut scanner = Scanner::new(&buffer);

    scanner.scan_tokens();

    Ok(0)
}

//todo: error handling
fn read_source_file(file_path: &str) -> Result<Vec<u8>> {
    let mut file = File::open(file_path).unwrap();
    let metadata = fs::metadata(file_path).unwrap();
    let mut buffer = vec![0; metadata.len() as usize];

    file.read_to_end(&mut buffer).expect("buffer overflow");

    Ok(buffer)
}

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, where_err: &str, message: &str) {
    println!("[Line {} ] Error {}: {}", line, where_err, message)
}
