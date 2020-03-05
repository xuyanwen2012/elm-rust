use std::fmt;

use unicode_xid::UnicodeXID;

use num_bigint::BigInt;

fn is_symbol(ch: char) -> bool {
    match ch {
        '&' | '!' | ':' | ',' | '.' | '=' | '/' | '>' | '<' | '-' | '|' | '+' | ';' | '*' => true,
        _ => false,
    }
}

fn is_ident_start(ch: char) -> bool {
    UnicodeXID::is_xid_start(ch) || ch == '_'
}

fn is_ident_continue(ch: char) -> bool {
    UnicodeXID::is_xid_continue(ch) || ch == '_'
}

fn is_dec_digit(ch: char) -> bool {
    ch.is_digit(10)
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    // Literals
    LitUnit,
    LitInt(BigInt),
    Name(String),
    // Keywords
    If,
    Then,
    Else,
    Let,
    In,
    Unit,
    Int,
    LiftN(i32),
    Foldp,
    Async,
    // Symbols
    BSlash, // \
    Colon,  // :
    LArrow, // ->
    Eq,     // =
    // Delimiters
    LParen, // (
    RParen, // )
    // BinOp
    Plus,
    Minus,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Token::*;
        match *self {
            LitUnit => write!(f, "'()'"),
            LitInt(ref value) => write!(f, "'{}'", value),
            Name(ref name) => write!(f, "'{}'", name),
            If => write!(f, "'if'"),
            Then => write!(f, "'then'"),
            Else => write!(f, "'else'"),
            Let => write!(f, "'let'"),
            In => write!(f, "'in'"),
            Unit => write!(f, "'unit'"),
            Int => write!(f, "'int'"),
            LiftN(ref value) => write!(f, "'lift_{}'", value),
            Foldp => write!(f, "'foldp'"),
            Async => write!(f, "'async'"),
            BSlash => write!(f, "'\\'"),
            Colon => write!(f, "':'"),
            LArrow => write!(f, "'->'"),
            Eq => write!(f, "'='"),
            LParen => write!(f, "'('"),
            RParen => write!(f, "')'"),
            Plus => write!(f, "'+'"),
            Minus => write!(f, "'-'"),
        }
    }
}

mod test {
    #[test]
    fn test_work() {}
}
