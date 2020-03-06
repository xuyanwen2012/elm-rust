use crate::tokens::Token;
use std::str::CharIndices;

use num_bigint::BigInt;
use num_traits::Num;

/// Some helper functions used in Lexer
fn is_symbol(ch: char) -> bool {
    match ch {
        '!' | ':' | ',' | '.' | '=' | '/' | '>' | '<' | '-' | '+' | '*' => true,
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
    pub fn new(source: &'input str) -> Self {
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

    /// Test a predicate against the next character in the source
    fn test_lookahead<F>(&self, mut pred: F) -> bool
    where
        F: FnMut(char) -> bool,
    {
        self.lookahead.map_or(false, |(_, ch)| pred(ch))
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
            "unit" => Token::Unit,
            "signal" => Token::Signal,
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

    /// The basic idea is that we will take a look at the next character, if any, and decide upon
    /// the next steps.
    fn next(&mut self) -> Option<Self::Item> {
        while let Some((start, ch)) = self.bump() {
            let end = start + 1;

            return Some(match ch {
                ch if is_symbol(ch) => {
                    let (end, symbol) = self.take_while(start, is_symbol);

                    match symbol {
                        "+" => Ok((start, Token::Plus, end)),
                        "-" => Ok((start, Token::Minus, end)),
                        "*" => Ok((start, Token::Mul, end)),
                        "/" => Ok((start, Token::Div, end)),
                        ":" => Ok((start, Token::Colon, end)),
                        "," => Ok((start, Token::Comma, end)),
                        "." => Ok((start, Token::Dot, end)),
                        "=" => Ok((start, Token::Eq, end)),
                        "->" => Ok((start, Token::LArrow, end)),
                        ">" => Ok((start, Token::Great, end)),
                        "<" => Ok((start, Token::Less, end)),
                        ">=" => Ok((start, Token::Geq, end)),
                        "<=" => Ok((start, Token::Leq, end)),
                        "==" => Ok((start, Token::EqEqual, end)),
                        "!=" => Ok((start, Token::Ne, end)),
                        _ => Err(LexicalError::UnexpectedCharacter),
                    }
                }
                '\\' => Ok((start, Token::BSlash, end)),
                '(' if self.test_lookahead(|c| c == ')') => {
                    self.bump();
                    Ok((start, Token::LitUnit, end))
                }
                '(' => Ok((start, Token::LParen, end)),
                ')' => Ok((start, Token::RParen, end)),
                ch if is_ident_start(ch) => Ok(self.lex_ident(start)),
                ch if is_dec_digit(ch) => Ok(self.lex_number(start)),
                ch if ch.is_whitespace() => continue,
                _ => Err(LexicalError::UnexpectedCharacter),
            });
        }

        // We have reached the end.
        None
    }
}

#[cfg(test)]
mod test {
    use super::Lexer;
    use crate::tokens::Token::*;
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
            "if then else let in int unit signal async foldp\n",
            If,
            Then,
            Else,
            Let,
            In,
            Int,
            Unit,
            Signal,
            Async,
            Foldp
        }
    }

    #[test]
    fn test_identifiers() {
        test! {
            "Elm4Rust_\n",
            Name("Elm4Rust_".to_owned())
        }
    }

    #[test]
    fn test_symbols() {
        test! {
            "() + - * / \\ , . -> = > < >= <= == !=\n",
            LitUnit,
            Plus,
            Minus,
            Mul,
            Div,
            BSlash,
            Comma,
            Dot,
            LArrow,
            Eq,
            Great,
            Less,
            Geq,
            Leq,
            EqEqual,
            Ne
        }
    }

    #[test]
    fn test_delimiters() {
        test! {
            "( )\n",
            LParen,
            RParen
        }
    }
}
