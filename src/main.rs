use std::io::{self, Write};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal, cursor, execute,
};

use numlite::ast::Stmt;
use numlite::eval::{eval, Env};
use numlite::lexer::Lexer;
use numlite::parser::Parser;

fn main() {
    println!("NumLite v0.0.5 — MATLAB ringan");
    println!("Ketik 'exit' untuk keluar.\n");

    let mut env: Env = Env::new();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let input = match read_line_with_arrows() {
            Ok(s) => s,
            Err(_) => break, // Esc ditekan
        };

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

fn read_line_with_arrows() -> Result<String, ()> {
    terminal::enable_raw_mode().unwrap();
    let mut buffer = Vec::new();
    let mut cursor_pos = 0;

    loop {
        // Perbaikan: Menambahkan `..` untuk mengabaikan field `kind` dan `state`
        if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read().unwrap() {
            
            // DETEKSI BUG LINUX: Backspace kadang terbaca sebagai Ctrl+h
            let is_backspace = matches!(code, KeyCode::Backspace) 
                || (matches!(code, KeyCode::Char('h')) && modifiers.contains(KeyModifiers::CONTROL));

            if is_backspace {
                if cursor_pos > 0 {
                    buffer.remove(cursor_pos - 1);
                    cursor_pos -= 1;
                    redraw_line(&buffer, cursor_pos);
                }
                continue;
            }

            match code {
                KeyCode::Enter => {
                    print!("\r\n");
                    break;
                }
                KeyCode::Char(c) => {
                    // Abaikan huruf biasa yang ditekan bersamaan dengan Ctrl
                    if modifiers.contains(KeyModifiers::CONTROL) {
                        continue; 
                    }
                    
                    if cursor_pos == buffer.len() {
                        buffer.push(c);
                        print!("{}", c);
                    } else {
                        buffer.insert(cursor_pos, c);
                        redraw_line(&buffer, cursor_pos + 1);
                    }
                    cursor_pos += 1;
                    io::stdout().flush().unwrap();
                }
                KeyCode::Left => {
                    if cursor_pos > 0 {
                        cursor_pos -= 1;
                        execute!(io::stdout(), cursor::MoveLeft(1)).unwrap();
                    }
                }
                KeyCode::Right => {
                    if cursor_pos < buffer.len() {
                        cursor_pos += 1;
                        execute!(io::stdout(), cursor::MoveRight(1)).unwrap();
                    }
                }
                KeyCode::Esc => {
                    terminal::disable_raw_mode().unwrap();
                    return Err(());
                }
                _ => {}
            }
        }
    }
    
    terminal::disable_raw_mode().unwrap();
    Ok(buffer.into_iter().collect())
}

fn redraw_line(buffer: &[char], cursor_pos: usize) {
    // Hapus baris saat ini dan gambar ulang
    print!("\r\x1b[K>> {}", buffer.iter().collect::<String>());
    let back = buffer.len() - cursor_pos;
    if back > 0 {
        execute!(io::stdout(), cursor::MoveLeft(back as u16)).unwrap();
    }
    io::stdout().flush().unwrap();
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
