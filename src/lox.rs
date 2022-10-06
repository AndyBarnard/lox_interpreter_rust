use std::fs;
// use std::fs::File;
use std::io::prelude::*;
use std::io::stdin;
use std::io::BufReader;
use std::process;
// use std::io;
// use std::env;

use crate::scanner::*;

#[derive(Debug)]
pub struct Lox {
    pub had_error: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self { had_error: false }
    }

    pub fn run_file(&self, file_path: &str) {
        println!("opening file...");
        // let file = File::open(&file_path).unwrap_or_else(|err| {
        //     eprintln!("{err}");
        //     process::exit(1);
        // });

        let file_source = fs::read_to_string(&file_path).unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        });

        // let reader = BufReader::new(&file);

        // for line in reader.lines() {
        //     println!("here's a line: {}", line.expect("error in run_file"));
        // }

        Lox::run(&file_source);

        if self.had_error {
            process::exit(1);
        }
    }

    pub fn run_prompt(&mut self) {
        let mut buffer = String::new();
        // let f = File::open(); need a stream to get the line?
        // let mut reader = BufReader::new();

        loop {
            print!("> ");
            let line = stdin().read_line(&mut buffer).unwrap(); //buffer contains the value of the line, and line is just the line num
                                                                // if line == null { break; }
            Self::run(&buffer);
            self.had_error = false;
        }
    }

    pub fn run(source: &str) {
        let scanner = Scanner::new(&source);
        let tokens = scanner.scan_tokens();

        for token in &tokens {
            println!("printing token: {:?}", token);
        }
    }

    pub fn error(&mut self, line: u32, message: &str) {
        self.report(line, "", message);
    }

    fn report(&mut self, line: u32, location: &str, message: &str) {
        eprintln!("[line {line}] Error {location}: message");
        self.had_error = true;
    }
}
