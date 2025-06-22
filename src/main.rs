use std::env;
use std::fs;

enum LexError {
    SingleTokenError { at: char, line: usize },
}

impl std::fmt::Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SingleTokenError { at, line } => {
                write!(f, "[line {}] Error: Unexpected character: {}", line, at)
            }
        }
    }
}

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
        return;
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
                for (line_num, line) in file_contents.lines().enumerate() {
                    for c in line.chars() {
                        if let Some(token) = Token::from_char(c) {
                            println!("{}", token);
                        } else if !c.is_whitespace() {
                            let err = LexError::SingleTokenError {
                                at: c,
                                line: line_num + 1,
                            };

                            println!("{}", err);
                        }
                    }
                }
            }

            println!("EOF  null");
        }
        _ => {
            eprintln!("Unknown command: {}", command);
        }
    }
}
