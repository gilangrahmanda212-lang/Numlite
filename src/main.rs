use std::io::{self, Write};

fn main() {
    println!("NumLite v0.0.1 — MATLAB ringan");
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
        
        println!("Anda mengetik: {}", input);
    }
}
