use crate::token::Token;

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

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
        }
    }

    pub fn scan(&mut self) {
        let mut line = 1;
        let mut has_error = false;

        let mut iter = self.source.chars().peekable();

        while let Some(char) = iter.next() {
            if char == '\n' {
                line += 1;

                continue;
            }

            match char {
                ',' => {
                    self.tokens.push(Token::Comma);
                }
                '.' => {
                    self.tokens.push(Token::Dot);
                }
                '-' => {
                    self.tokens.push(Token::Minus);
                }
                '+' => {
                    self.tokens.push(Token::Plus);
                }
                ';' => {
                    self.tokens.push(Token::Semicolon);
                }
                '/' => {
                    self.tokens.push(Token::Slash);
                }
                '*' => {
                    self.tokens.push(Token::Star);
                }
                '(' => {
                    self.tokens.push(Token::LeftParen);
                }
                ')' => {
                    self.tokens.push(Token::RightParen);
                }
                '{' => {
                    self.tokens.push(Token::LeftBrace);
                }
                '}' => {
                    self.tokens.push(Token::RightBrace);
                }
                '=' => {
                    if iter.peek() == Some(&'=') {
                        self.tokens.push(Token::EqualEqual);
                        iter.next();
                    } else {
                        self.tokens.push(Token::Equal);
                    }
                }
                _ => {
                    has_error = true;
                    println!("{}", LexError::SingleTokenError { at: char, line });
                }
            }
        }

        self.tokens.iter().for_each(|token| token.print());

        println!("EOF  null");

        if has_error {
            std::process::exit(65);
        }
    }
}
