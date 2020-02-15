use std::fmt::{self, Write};

#[derive(Clone, Debug, PartialEq)]
pub enum Tok {
    Eof,
    Name,
    LitInt(i32),
    LitFloat(f64),
    If,
    Else,
    Then,
}

impl fmt::Display for Tok {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tok::Eof => f.write_str("Eof"),
            Tok::Name => f.write_str("Name"),
            Tok::LitInt(value) => write!(f, "'{}'", value),
            Tok::LitFloat(value) => write!(f, "'{}'", value),
            Tok::If => f.write_str("If"),
            Tok::Else => f.write_str("Else"),
            Tok::Then => f.write_str("Then"),
        }
    }
}
