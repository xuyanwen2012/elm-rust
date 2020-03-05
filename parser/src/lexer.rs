use std::fmt;
use std::option::Option::Some;
use std::str::CharIndices;

use num_bigint::BigInt;

fn is_symbol(ch: char) -> bool {
    match ch {
        '&' | '!' | ':' | ',' | '.' | '=' | '/' | '>' | '<' | '-' | '|' | '+' | ';' | '*' => true,
        _ => false,
    }
}

fn is_ident_start(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

fn is_ident_continue(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

fn is_dec_digit(ch: char) -> bool {
    ch.is_digit(10)
}

pub enum LexicalError {
    // Not possible
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

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

/// An iterator over a source string that yields `Token`s for subsequent use by
/// the parser
pub struct Lexer<'input> {
    lookahead: Option<(usize, char)>,
    chars: CharIndices<'input>,
}

impl<'input> Lexer<'input> {
    /// Create a new lexer from the source string
    fn new(input: &'input str) -> Self {
        let mut chars = input.char_indices();

        Lexer {
            lookahead: chars.next(),
            chars,
        }
    }

    fn lookahead(&self) -> Option<(usize, char)> {
        self.lookahead.map(|(index, ch)| (index + 1, ch))
    }

    /// Bump the current position in the source string by one character, returning the current
    /// character and byte position.
    fn bump(&mut self) -> Option<(usize, char)> {
        let current = self.lookahead();
        self.lookahead = self.chars.next();
        current
    }
}

/// Implement iterator pattern for the get_tok function. Calling the next element in the
/// iterator will yield the next lexical token.
impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        // Take a look at the next character, if any, and decide upon the next steps.
        while let Some((start, ch)) = self.bump() {
            println!("at {:?}: {:?}", start, ch);
            // First check identifier:
            if is_ident_start(ch) {}
        }

        None
    }
}

mod test {
    use super::Lexer;

    #[test]
    fn test_work() {
        let source = "if x else 1 then 2";
        let lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.collect();
    }
}
