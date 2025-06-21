use std::env;
use std::fs;

enum Token {
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
}

impl Token {
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            ',' => Some(Self::Comma),
            '.' => Some(Self::Dot),
            '-' => Some(Self::Minus),
            '+' => Some(Self::Plus),
            ';' => Some(Self::Semicolon),
            '/' => Some(Self::Slash),
            '*' => Some(Self::Star),
            '(' => Some(Self::LeftParen),
            ')' => Some(Self::RightParen),
            '{' => Some(Self::LeftBrace),
            '}' => Some(Self::RightBrace),
            _ => None,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Comma => write!(f, "COMMA , null"),
            Self::Dot => write!(f, "DOT . null"),
            Self::Minus => write!(f, "MINUS - null"),
            Self::Plus => write!(f, "PLUS + null"),
            Self::Semicolon => write!(f, "SEMICOLON ; null"),
            Self::Slash => write!(f, "SLASH / null"),
            Self::Star => write!(f, "STAR * null"),
            Self::LeftParen => write!(f, "LEFT_PAREN ( null"),
            Self::RightParen => write!(f, "RIGHT_PAREN ) null"),
            Self::LeftBrace => write!(f, "LEFT_BRACE {{ null"),
            Self::RightBrace => write!(f, "RIGHT_BRACE }} null"),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!("Failed to read file {}", filename);
                String::new()
            });

            if !file_contents.is_empty() {
                file_contents
                    .chars()
                    .map(Token::from_char)
                    .for_each(|token| {
                        if let Some(token) = token {
                            println!("{}", token);
                        } else {
                            eprintln!(
                                "Invalid character in file: {}",
                                token.unwrap_or(Token::LeftParen)
                            );
                        }
                    });
            }

            println!("EOF  null");
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
