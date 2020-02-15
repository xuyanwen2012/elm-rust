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
        assert!(calculator1::TermParser::new().parse("22").is_ok());
        assert!(calculator1::TermParser::new().parse("(22)").is_ok());
        assert!(calculator1::TermParser::new().parse("((((22))))").is_ok());
        assert!(calculator1::TermParser::new().parse("((22)").is_err());
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
