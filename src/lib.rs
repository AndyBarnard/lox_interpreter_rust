// use std::fs;
// use std::fs::File;
// use std::io;
// use std::io::prelude::*;
// use std::io::stdin;
// use std::io::BufReader;
use std::env;
use std::process;

// use crate::scanner::*;
use crate::lox::*;

pub mod lox;
pub mod scanner;
pub mod token;
pub mod tokentype;

pub fn init() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        eprintln!("Too many args");

        process::exit(1);
    } else if args.len() == 2 {
        let file_path = &args[1];

        if let '/' = file_path.chars().next().unwrap() {
            let lox = Lox::new();
            lox.run_file(&file_path);
            println!("running file...");
        } else {
            let mut current_path = env::current_dir().unwrap_or_else(|err| {
                eprintln!("error getting current_dir with error: {}", err);
                process::exit(1);
            });

            current_path.push(file_path);

            let absolute_path = current_path
                .into_os_string()
                .into_string()
                .unwrap_or_else(|_| {
                    eprintln!("Unprintable OsString error");

                    process::exit(1);
                });

            let lox = Lox::new();
            lox.run_file(&absolute_path);
        }
        // let bytes = contents.as_bytes();
        // io::stdout().write(&bytes);
        // println!("Text in provided file:\n{contents}");
    } else {
        let mut lox = Lox::new();
        lox.run_prompt(); //interactive lox CLI
    }
}
