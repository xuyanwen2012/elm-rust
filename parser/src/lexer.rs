use std::fmt;
use std::option::Option::Some;
use std::str::CharIndices;

use num_bigint::BigInt;
use num_traits::Num;

fn is_symbol(ch: char) -> bool {
    match ch {
        '&' | '!' | ':' | ',' | '.' | '=' | '/' | '>' | '<' | '-' | '|' | '+' | ';' | '*' => true,
        _ => false,
    }
}

/// In our case, we can assume the source code is ASCII.
fn is_ident_start(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

fn is_ident_continue(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

fn is_dec_digit(ch: char) -> bool {
    ch.is_digit(10)
}

#[derive(Debug)]
pub enum LexicalError {
    UnexpectedCharacter,
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
    source: &'input str,
    lookahead: Option<(usize, char)>,
    chars: CharIndices<'input>,
}

impl<'input> Lexer<'input> {
    /// Create a new lexer from the source string
    fn new(source: &'input str) -> Self {
        let mut chars = source.char_indices();

        Lexer {
            source,
            lookahead: chars.next(),
            chars,
        }
    }

    /// Bump the current position in the source string by one character, returning the current
    /// character and byte position.
    fn bump(&mut self) -> Option<(usize, char)> {
        let current = self.lookahead;
        self.lookahead = self.chars.next();
        current
    }

    /// Return a slice of the source string.
    fn slice(&self, start: usize, end: usize) -> &'input str {
        &self.source[start..end]
    }

    /// Consume characters while the predicate matches for the current character, then return the
    /// consumed slice and the end byte position.
    fn take_while<F>(&mut self, start: usize, mut keep_going: F) -> (usize, &'input str)
    where
        F: FnMut(char) -> bool,
    {
        self.take_until(start, |ch| !keep_going(ch))
    }

    /// Consume characters until the predicate matches for the next character in the lookahead,
    /// then return the consumed slice and the end byte position.
    fn take_until<F>(&mut self, start: usize, mut terminate: F) -> (usize, &'input str)
    where
        F: FnMut(char) -> bool,
    {
        let mut str = String::new();

        while let Some((end, ch)) = self.lookahead {
            if terminate(ch) {
                return (end, self.slice(start, end));
            } else {
                str.push(ch);
                self.bump();
            }
        }

        (0, self.chars.as_str())
    }

    /// Consume an identifier, or they might be keywords.
    fn lex_ident(&mut self, start: usize) -> (usize, Token, usize) {
        let (end, ident) = self.take_while(start, is_ident_continue);

        let token = match ident {
            "if" => Token::If,
            "else" => Token::Else,
            "then" => Token::Then,
            "let" => Token::Let,
            "in" => Token::In,
            "int" => Token::Int,
            "async" => Token::Async,
            // TODO liftN
            "foldp" => Token::Foldp,
            ident => Token::Name(ident.to_string()),
        };

        (start, token, end)
    }

    /// Consume a decimal literal
    fn lex_number(&mut self, start: usize) -> (usize, Token, usize) {
        let (end, src) = self.take_while(start, is_dec_digit);
        let value = BigInt::from_str_radix(src, 10).unwrap();
        (start, Token::LitInt(value), end)
    }
}

/// Implement iterator pattern for the get_tok function. Calling the next element in the
/// iterator will yield the next lexical token.
impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        // Take a look at the next character, if any, and decide upon the next steps.
        while let Some((start, ch)) = self.bump() {
            let end = start + 1;

            return Some(match ch {
                '\\' => Ok((start, Token::BSlash, end)),
                ch if is_ident_start(ch) => Ok(self.lex_ident(start)),
                ch if is_dec_digit(ch) => Ok(self.lex_number(start)),
                ch if ch.is_whitespace() => continue,
                _ => Err(LexicalError::UnexpectedCharacter),
            });
        }

        None
    }
}

mod test {
    use super::Lexer;
    use super::Token::*;
    use num_bigint::BigInt;

    macro_rules! test {
        ($source:expr, $($tokens:expr),+) => {{
            let lexer = Lexer::new($source);
            let lexed_tokens: Vec<_> = lexer.map(|x| x.unwrap().1).collect();

            let expected_tokens = vec![$($tokens), +];

            assert_eq!(lexed_tokens, expected_tokens);
        }};
    }

    #[test]
    fn test_numbers() {
        // Always have a newline character at the end of file.
        test! {
            "1 22 333 4444\n",
            LitInt(BigInt::from(1)),
            LitInt(BigInt::from(22)),
            LitInt(BigInt::from(333)),
            LitInt(BigInt::from(4444))
        }
    }

    #[test]
    fn test_keywords() {
        test! {
            "if then else let in int async foldp\n",
            If,
            Then,
            Else,
            Let,
            In,
            Int,
            Async,
            Foldp
        }
    }
}
