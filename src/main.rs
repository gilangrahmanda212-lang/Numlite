use std::io::{self, Write};

use numlite::ast::Stmt;
use numlite::eval::{eval, Env};
use numlite::lexer::Lexer;
use numlite::parser::Parser;

fn main() {
    println!("NumLite (Dalam tahap pengembangan)");
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

        let mut lexer = Lexer::new(input);
        let tokens = match lexer.tokenize() {
            Ok(t) => t,
            Err(e) => {
                println!("Lexer error: {}", e);
                continue;
            }
        };

        let mut parser = Parser::new(tokens);
        let stmt = match parser.parse_stmt() {
            Ok(s) => s,
            Err(e) => {
                println!("Parser error: {}", e);
                continue;
            }
        };

        match stmt {
            Stmt::Assign { name, value } => match eval(&value, &env) {
                Ok(v) => {
                    println!("{} =", name);
                    print_value(&v);
                    env.insert(name, v);
                }
                Err(e) => println!("Eval error: {}", e),
            },
            Stmt::Expr(expr) => match eval(&expr, &env) {
                Ok(v) => {
                    println!("ans =");
                    print_value(&v);
                }
                Err(e) => println!("Eval error: {}", e),
            },
        }
    }
}

fn print_value(v: &numlite::value::Value) {
    use numlite::value::Value;
    match v {
        Value::Scalar(n) => println!("  {}", format_num(*n)),
        Value::Matrix(m) => {
            if m.rows == 0 || m.cols == 0 {
                println!("  []");
                return;
            }
            let mut col_widths = vec![0usize; m.cols];
            for c in 0..m.cols {
                for r in 0..m.rows {
                    let len = format_num(m.get(r, c)).len();
                    if len > col_widths[c] {
                        col_widths[c] = len;
                    }
                }
            }
            for r in 0..m.rows {
                print!("  ");
                for c in 0..m.cols {
                    if c > 0 {
                        print!("  ");
                    }
                    let s = format_num(m.get(r, c));
                    print!("{:>width$}", s, width = col_widths[c]);
                }
                println!();
            }
        }
    }
}

fn format_num(n: f64) -> String {
    if n.fract() == 0.0 && n.abs() < 1e15 {
        format!("{}", n as i64)
    } else {
        let s = format!("{:.4}", n);
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}
