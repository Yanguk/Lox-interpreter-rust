pub enum Token {
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
    Equal,
    EqualEqual,
}

// impl Token {
//     pub fn from_char(c: char) -> Option<Self> {
//         match c {
//             ',' => Some(Self::Comma),
//             '.' => Some(Self::Dot),
//             '-' => Some(Self::Minus),
//             '+' => Some(Self::Plus),
//             ';' => Some(Self::Semicolon),
//             '/' => Some(Self::Slash),
//             '*' => Some(Self::Star),
//             '(' => Some(Self::LeftParen),
//             ')' => Some(Self::RightParen),
//             '{' => Some(Self::LeftBrace),
//             '}' => Some(Self::RightBrace),
//             '==' => Some(Self::RightBrace),
//             _ => None,
//         }
//     }
// }

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

            Self::Equal => write!(f, "EQUAL = null"),
            Self::EqualEqual => write!(f, "EQUAL_EQUAL == null"),
        }
    }
}

impl Token {
    pub fn print(&self) {
        println!("{}", self);
    }
}
