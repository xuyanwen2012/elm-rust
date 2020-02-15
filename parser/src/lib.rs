pub mod ast;
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
    fn test_lalrpop_calculator() {
        use super::calculator1;

        assert!(calculator1::ExprParser::new().parse("22").is_ok());
        assert!(calculator1::ExprParser::new().parse("(22)").is_ok());
        assert!(calculator1::ExprParser::new().parse("((((22))))").is_ok());
        assert!(calculator1::ExprParser::new().parse("((22)").is_err());
    }

    #[test]
    fn test_lalrpop_calculator4() {
        use super::calculator1;

        let expr = calculator1::ExprParser::new()
            .parse("22 * 44 + 66")
            .unwrap();
        assert_eq!(&format!("{:?}", expr), "((22 * 44) + 66)");
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
