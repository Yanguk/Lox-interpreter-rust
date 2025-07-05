mod scanner;
mod token;

use scanner::Scanner;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap();
            let mut scanner = Scanner::new(&file_contents);

            scanner.scan();
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
