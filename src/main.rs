
use std::io::{self, Write};

use numlite::eval::eval;
use numlite::lexer::Lexer;
use numlite::parser::Parser;

fn main() {
    println!("NumLite v0.0.2 — MATLAB ringan");
    println!("Ketik 'exit' untuk keluar.\n");

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();
        if input == "exit" {
            break;
        }
        if input.is_empty() {
            continue;
        }

        // 1. Tokenize
        let mut lexer = Lexer::new(input);
        let tokens = match lexer.tokenize() {
            Ok(t) => t,
            Err(e) => {
                println!("Lexer error: {}", e);
                continue;
            }
        };

        // 2. Parse
        let mut parser = Parser::new(tokens);
        let ast = match parser.parse() {
            Ok(a) => a,
            Err(e) => {
                println!("Parser error: {}", e);
                continue;
            }
        };

        // 3. Eval
        match eval(&ast) {
            Ok(value) => println!("ans = {}", value),
            Err(e) => println!("Eval error: {}", e),
        }
    }
}
