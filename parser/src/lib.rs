pub mod ast;
pub mod token;

#[macro_use]
extern crate lalrpop_util;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(
    pub elm
);

#[cfg(test)]
mod tests {
    use crate::token::Tok;

    #[test]
    fn test_literals() {
        use super::elm;

        assert!(elm::TermParser::new().parse("42").is_ok());
    }
}
