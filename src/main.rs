mod errors;
mod scanner;
mod token;

extern crate core;

use crate::errors::Error;
use crate::scanner::Scanner;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::process::exit;
use std::{env, fs};

type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let x = match args.len() {
        2 => runFile(&args[1]),
        1 => runPrompt(),
        _ => panic!("fucked up"),
    }?;

    exit(x);
}

fn runPrompt() -> Result<i32> {
    loop {
        println!(">");
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("failed to read line");

        run(&line)?;
    }

    Ok(0)
}

fn run(line: &str) -> Result<()> {
    let mut scanner = Scanner::new(line.as_bytes());

    let tokens = scanner.scan_tokens();

    for token in &tokens.to_vec() {
        println!("{}", token);
    }

    Ok(())
}

fn runFile(file_path: &str) -> Result<i32> {
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
    println!(
        "{}",
        format!("[line {} ] Error {}: {}", line, where_err, message)
    )
}
