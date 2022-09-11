use std::env;
use std::process;
// use std::fs;
// use std::fs::File;
// use std::io;
// use std::io::prelude::*;
// use std::io::stdin;
// use std::io::BufReader;

use crate::lox::*;
// use crate::scanner::*;

mod lox;
mod scanner;
mod token;
mod tokentype;

pub fn init() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    if args.len() > 2 {
        eprintln!("Too many args");
        process::exit(1);
    } else if args.len() == 2 {
        let file_path = &args[1];

        if let '/' = file_path.chars().next().unwrap() {
            let lox = Lox::new();
            Lox::run_file(&lox, &file_path);
        } else {
            let mut current_path = env::current_dir().unwrap_or_else(|err| {
                eprintln!("error getting current_dir with error: {}", err);
                process::exit(1);
            });

            current_path.push(file_path);

            //TODO: OsString to String
            let absolute_path = current_path
                .into_os_string()
                .into_string()
                .unwrap_or_else(|err| {
                    eprintln!("OsString error");
                    process::exit(1);
                });

            let lox = Lox::new();
            Lox::run_file(&lox, &absolute_path);
        }
        // let bytes = contents.as_bytes();
        // io::stdout().write(&bytes);
        // println!("Text in provided file:\n{contents}");
    } else {
        let mut lox = Lox::new();
        Lox::run_prompt(&mut lox); //interactive lox CLI
    }
}
