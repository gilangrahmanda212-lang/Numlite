use std::io::{self, Write};

use numlite::ast::Stmt;
use numlite::eval::{eval, Env};
use numlite::lexer::Lexer;
use numlite::parser::Parser;

fn main() {
    println!("NumLite v0.0.3 — MATLAB ringan");
    println!("Ketik 'exit' untuk keluar.\n");

    let mut env: Env = Env::new();

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
        let stmt = match parser.parse_stmt() {
            Ok(s) => s,
            Err(e) => {
                println!("Parser error: {}", e);
                continue;
            }
        };

        // 3. Eval
        match stmt {
            Stmt::Assign { name, value } => match eval(&value, &env) {
                Ok(v) => {
                    env.insert(name.clone(), v);
                    println!("{} = {}", name, v);
                }
                Err(e) => println!("Eval error: {}", e),
            },
            Stmt::Expr(expr) => match eval(&expr, &env) {
                Ok(v) => println!("ans = {}", v),
                Err(e) => println!("Eval error: {}", e),
            },
        }
    }
}
