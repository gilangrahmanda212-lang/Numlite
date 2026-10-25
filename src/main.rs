use std::io::{self, Write};
use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyModifiers}, terminal, cursor, execute};

use numlite::eval::{exec_stmt, Env};
use numlite::lexer::Lexer;
use numlite::parser::Parser;
fn main() {
    println!("NumLite v0.0.6 — MATLAB ringan (Multiline)");
    println!("Ketik 'exit' untuk keluar.\n");

    let mut env: Env = Env::new();

    loop {
        let mut input = String::new();
        let mut is_multiline = false;

        loop {
            if is_multiline { print!("... "); } else { print!(">> "); }
            io::stdout().flush().unwrap();

            let line = match read_line_with_arrows() {
                Ok(s) => s,
                Err(_) => return,
            };

            let trimmed = line.trim();
            if trimmed == "exit" && !is_multiline { return; }

                        if trimmed.starts_with("for ") || trimmed.starts_with("if ") { 
                is_multiline = true; 
            }
            if trimmed == "end" { is_multiline = false; }

            input.push_str(trimmed);
            input.push('\n');

            if !is_multiline && !trimmed.is_empty() { break; }
        }

        let input = input.trim();
        if input.is_empty() { continue; }

        let mut lexer = Lexer::new(input);
        let tokens = match lexer.tokenize() {
            Ok(t) => t,
            Err(e) => { println!("Lexer error: {}", e); continue; }
        };

        let mut parser = Parser::new(tokens);
        
        loop {
            if parser.is_at_eof() { break; }
            match parser.parse_stmt() {
                Ok(stmt) => {
                    if let Err(e) = exec_stmt(&stmt, &mut env) {
                        println!("Eval error: {}", e);
                    }
                }
                Err(e) => { 
                    if e != "Eof" { println!("Parser error: {}", e); }
                    break; 
                }
            }
        }
    }
}

fn read_line_with_arrows() -> Result<String, ()> {
    terminal::enable_raw_mode().unwrap();
    let mut buffer = Vec::new();
    let mut cursor_pos = 0;

    loop {
        if let Event::Key(KeyEvent { code, modifiers, .. }) = event::read().unwrap() {
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
                    if modifiers.contains(KeyModifiers::CONTROL) { continue; }
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
    print!("\r\x1b[K>> {}", buffer.iter().collect::<String>());
    let back = buffer.len() - cursor_pos;
    if back > 0 {
        execute!(io::stdout(), cursor::MoveLeft(back as u16)).unwrap();
    }
    io::stdout().flush().unwrap();
}
