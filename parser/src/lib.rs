pub mod ast;
pub mod lexer;
pub mod token;

#[macro_use]
extern crate lalrpop_util;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    pub calculator1
);

#[cfg(test)]
mod tests {
    use crate::token::Tok;

    #[test]
    fn test_literal() {}
}
