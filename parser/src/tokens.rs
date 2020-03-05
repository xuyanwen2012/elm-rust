use num_bigint::BigInt;
use std::fmt;

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
    Comma,  // ,
    Colon,  // :
    LArrow, // ->
    Eq,     // =
    // Delimiters
    LParen, // (
    RParen, // )
    // BinOp
    Plus,
    Minus,
    Mul,
    Div,
    Great,   // >
    Less,    // <
    Leq,     // <=
    Geq,     // >=
    EqEqual, // ==
    Ne,      // >=
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
            Comma => write!(f, "','"),
            Mul => write!(f, "'*'"),
            Div => write!(f, "'/'"),
            Great => write!(f, "'>'"),
            Less => write!(f, "'<'"),
            Leq => write!(f, "'<='"),
            Geq => write!(f, "'>='"),
            EqEqual => write!(f, "'=='"),
            Ne => write!(f, "'!='"),
        }
    }
}
