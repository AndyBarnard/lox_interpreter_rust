use std::fs;
use std::io::stdin;
use std::process;

use crate::scanner::*;
use crate::token::*;
use crate::tokentype::*;
use crate::parser::*;

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

        self.run(&file_source);

        if self.had_error {
            process::exit(1);
        }
    }

    //TODO: fix run_prompt
    pub fn run_prompt(&mut self) {
        let mut buffer = String::new();
        // let f = File::open(); need a stream to get the line?
        // let mut reader = BufReader::new();

        loop {
            print!("> ");
            let line = stdin().read_line(&mut buffer).unwrap(); //buffer contains the value of the line, and line is just the line num
                                                                // if line == null { break; }
            self.run(&buffer);
            self.had_error = false;
        }
    }

    pub fn run(&self, source: &str) {
        let scanner = Scanner::new(&source);
        let tokens = scanner.scan_tokens();

        //code to print scanned tokens, commented out bc not necessary
        // for token in &tokens {
        //     println!("Printing token: {:?}", token);
        // }

        let parser = Parser::new(tokens);
        let expression = parser.parse();

        if self.had_error {
            return;
        }

        // println!("Here he inits the AstPrinter with expression: {}", expression);
    }

    //TODO: maybe have line: Option<u32>?
    // pub fn error(&mut self, line: u32, message: &str) {
    //     self.report(line, "", message);
    // }

    //TODO: make report a macro to avoid the messy formatting as in error() below
    fn report(&mut self, line: u32, location: &str, message: &str) {
        eprintln!("[line {line}] Error {location} {message}");
        self.had_error = true;
    }

    pub fn error(&self, token: &Token, message: &str) {
        if *token.token_type == TokenType::Eof {
            self.report(token.line, " at end", message);
        } else {
            self.report(token.line, "", message);
        }
    }
}
