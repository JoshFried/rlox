mod scanner;
mod token;

extern crate core;

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::process::exit;
use std::{env, fs};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let x = match args.len() {
        2 => runFile(&args[1]),
        1 => runPrompt(),
        _ => panic!("fucked up"),
    }?;

    exit(x);
}

fn runPrompt() {
    loop {
        println!(">");
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("failed to read line");

        run(line)?;
    }
}

#[derive(Debug)]
struct Scanner();

#[derive(Debug)]
struct Token();

impl Token {
    fn new() -> Self {
        Self {}
    }
}

impl Scanner {
    pub(crate) fn scan_tokens(&self) -> Vec<Token> {
        vec![]
    }
}

impl Scanner {
    fn new() -> Self {
        Self {}
    }
}

fn run(line: String) -> Result<(), Error> {
    let scanner = Scanner::new();

    let tokens = scanner.scan_tokens();

    for token in &tokens {
        println!("{:?}", token);
    }

    Ok(())
}

fn runFile(file_path: &str) -> Result<i32, Error> {
    if !Path::new(base).exists() {
        return Ok(-1);
    }

    let buffer = read_source_file(file_path)?;

    Ok(0)
}

fn read_source_file(file_path: &str) -> Result<Vec<u8>, Error> {
    let mut file = File::open(file_path)?;
    let metadata = fs::metadata(file_path)?;
    let mut buffer = vec![0; metadata.len() as usize];
    file.read(&mut buffer).expect("buffer overflow");

    Ok(buffer)
}

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, where_err: &str, message: &str) {
    println!(format!("[line {} ] Error {}: {}", line, where_err, message))
}
