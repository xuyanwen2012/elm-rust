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
    fn test_literal() {
        use super::elm;

        assert!(elm::ExprParser::new().parse("42").is_ok());
    }

    #[test]
    fn test_binop() {
        use super::elm;

        let expr = elm::ExprParser::new().parse("1 + 2 * 3").unwrap();
        assert_eq!(&format!("{:?}", expr), "(1 + (2 * 3))")
    }

    #[test]
    fn test_if() {
        use super::elm;

        let expr = elm::ExprParser::new().parse("if 1 then 2 else 3").unwrap();
        assert_eq!(&format!("{:?}", expr), "if 1 then 2 else 3");
    }
}
