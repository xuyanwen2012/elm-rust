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

        // Numbers
        assert!(elm::ExprParser::new().parse("42").is_ok());

        // Identifiers
        assert!(elm::ExprParser::new().parse("point").is_ok());
        assert!(elm::ExprParser::new().parse("viewNames1").is_ok());
        assert!(elm::ExprParser::new().parse("twentyFour").is_ok());
    }

    #[test]
    fn test_binop() {
        use super::elm;

        // Literals
        let expr = elm::ExprParser::new().parse("1 + 2 * 3").unwrap();
        assert_eq!(&format!("{:?}", expr), "(1 + (2 * 3))");

        // Literal with Identifiers
        let expr = elm::ExprParser::new().parse("a + b * 3").unwrap();
        assert_eq!(&format!("{:?}", expr), "(\"a\" + (\"b\" * 3))");

        // Compare
        let expr = elm::ExprParser::new().parse("a == b").unwrap();
        assert_eq!(&format!("{:?}", expr), "(\"a\" == \"b\")");

        let expr = elm::ExprParser::new()
            .parse("1 + 2 * 3 == 3 * 2 + 1")
            .unwrap();
        assert_eq!(&format!("{:?}", expr), "((1 + (2 * 3)) == ((3 * 2) + 1))")
    }

    #[test]
    fn test_if() {
        use super::elm;

        // Simple if else
        let expr = elm::ExprParser::new().parse("if 1 then 2 else 3").unwrap();
        assert_eq!(&format!("{:?}", expr), "if ( 1 ) then { 2 } else { 3 }");

        // Complex if else
        let expr = elm::ExprParser::new()
            .parse("if 1 + 2 then 2 else 3")
            .unwrap();
        assert_eq!(
            &format!("{:?}", expr),
            "if ( (1 + 2) ) then { 2 } else { 3 }"
        );

        // More Complex if else
        let expr = elm::ExprParser::new()
            .parse("if 1 + 2 then 2 - 3 else 3 + 4 + 5")
            .unwrap();
        assert_eq!(
            &format!("{:?}", expr),
            "if ( (1 + 2) ) then { (2 - 3) } else { ((3 + 4) + 5) }"
        );

        // Very tricky pred
        let expr = elm::ExprParser::new()
            .parse("if (if 1 then 2 else 3) then 2 else 3")
            .unwrap();
        assert_eq!(
            &format!("{:?}", expr),
            "if ( if ( 1 ) then { 2 } else { 3 } ) then { 2 } else { 3 }"
        );

        // With Identifiers
        let expr = elm::ExprParser::new()
            .parse("if powerLevel > 9000 then 1111 else 2222")
            .unwrap();
        assert_eq!(
            &format!("{:?}", expr),
            "if ( (\"powerLevel\" > 9000) ) then { 1111 } else { 2222 }"
        );

        // If-else-if
        let expr = elm::ExprParser::new()
            .parse(
                "\
                  if key == 40 then
                      n + 1
                
                  else if key == 38 then
                      n - 1
                
                  else
                      n",
            )
            .unwrap();
        assert_eq!(
            &format!("{:?}", expr),
            "if ( (\"key\" == 40) ) then { (\"n\" + 1) } else { if ( (\"key\" == 38) ) then { (\"n\" - 1) } else { \"n\" } }"
        );
    }

    #[test]
    fn test_let() {
        use super::elm;

        let expr = elm::ExprParser::new().parse("let x = 1 + 2 in x").unwrap();
        assert_eq!(&format!("{:?}", expr), "let \"x\" = (1 + 2) in \"x\"");

        assert!(elm::ExprParser::new().parse("let 1 = 1 + 2 in x").is_err());
        assert!(elm::ExprParser::new().parse("let 1 = 1 + 2").is_err())
    }
}
