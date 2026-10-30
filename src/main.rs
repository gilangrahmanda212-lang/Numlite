use std::env;
use std::io::{self, Write};
use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyModifiers}, terminal, cursor, execute};

use numlite::ast::Stmt;
use numlite::eval::{exec_stmt, Env};
use numlite::lexer::Lexer;
use numlite::parser::Parser;
use numlite::value::Value;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // Jika ada argumen --cli, jalankan mode terminal
    if args.len() > 1 && args[1] == "--cli" {
        run_cli();
    } else {
        run_gui();
    }
}

fn run_gui() {
    // Muat ikon dari file PNG yang ada di folder utama proyek
    // Jika Anda belum punya icon.png, hapus 3 baris ini dan .with_icon(icon) di bawah
    let icon = eframe::icon_data::from_png_bytes(
        &include_bytes!("../icon.png")[..]
    ).expect("Gagal memuat icon.png. Pastikan file icon.png ada di folder utama proyek!");

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 700.0])
            .with_title("NumLite - MATLAB Ringan")
            .with_icon(icon), // Pasang ikon di sini
        ..Default::default()
    };
    
    let _ = eframe::run_native(
        "NumLite",
        options,
        Box::new(|cc| Box::new(numlite::app::NumLiteApp::new(cc))),
    );
}

fn run_cli() {
    println!("NumLite v0.0.6 — Mode Terminal (CLI)");
    println!("Ketik 'exit' untuk keluar.\n");

    let mut env: Env = Env::new();
    env.insert("pi".to_string(), Value::Scalar(std::f64::consts::PI));

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

            if trimmed.starts_with("for ") || trimmed.starts_with("if ") { is_multiline = true; }
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
        
        while !parser.is_at_eof() {
            match parser.parse_stmt() {
                Ok(stmt) => {
                    if let Stmt::Plot { .. } = stmt {
                        println!("[Grafik diabaikan di Mode CLI]");
                    } else {
                        let mut captured_output = String::new();
                        if let Err(e) = exec_stmt(&stmt, &mut env, &mut captured_output, false) {
                            println!("Eval error: {}", e);
                        } else {
                            print!("{}", captured_output);
                        }
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
                KeyCode::Enter => { print!("\r\n"); break; }
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
                    if cursor_pos > 0 { cursor_pos -= 1; execute!(io::stdout(), cursor::MoveLeft(1)).unwrap(); }
                }
                KeyCode::Right => {
                    if cursor_pos < buffer.len() { cursor_pos += 1; execute!(io::stdout(), cursor::MoveRight(1)).unwrap(); }
                }
                KeyCode::Esc => { terminal::disable_raw_mode().unwrap(); return Err(()); }
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
    if back > 0 { execute!(io::stdout(), cursor::MoveLeft(back as u16)).unwrap(); }
    io::stdout().flush().unwrap();
}