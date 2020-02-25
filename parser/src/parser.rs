use crate::elm;

macro_rules! do_lalr_parsing {
    ($input: expr) => {
        elm::ExprParser::new().parse($input).unwrap()
    };
}

#[cfg(test)]
mod tests {
    use super::elm;

    #[test]
    fn test_constant() {
        assert!(elm::ExprParser::new().parse("42").is_ok());
        assert!(elm::ExprParser::new().parse("()").is_ok());
        assert!(elm::ExprParser::new().parse("x").is_ok());
        assert!(elm::ExprParser::new().parse("())").is_err());
    }

    #[test]
    fn test_abs() {
        assert!(elm::ExprParser::new().parse("\\ -> 1").is_err());
        assert!(elm::ExprParser::new().parse("\\ 1 -> 1").is_err());
        assert!(elm::ExprParser::new().parse("1 -> 1").is_err());

        let expr = elm::ExprParser::new().parse("\\ x y z -> ()").unwrap();
        assert_eq!(&format!("{:?}", expr), "(\\ \"x\" \"y\" \"z\" -> ())");
    }

    #[test]
    fn test_app() {
        assert!(elm::ExprParser::new().parse("1 1").is_ok());

        let expr = elm::ExprParser::new().parse("() ()").unwrap();
        assert_eq!(&format!("{:?}", expr), "(() ())");

        // Note: this one should be checked by type checker
        let expr = elm::ExprParser::new().parse("\\ x -> 1 1").unwrap();
        assert_eq!(&format!("{:?}", expr), "(\\ \"x\" -> (1 1))");
    }

    #[test]
    fn test_binop() {
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
        // Single
        let expr = elm::ExprParser::new().parse("let x = 1 + 2 in x").unwrap();
        assert_eq!(&format!("{:?}", expr), "let \"x\" = (1 + 2) in \"x\"");

        assert!(elm::ExprParser::new().parse("let 1 = 1 + 2 in x").is_err());
        assert!(elm::ExprParser::new().parse("let 1 = 1 + 2").is_err());

        // Multiple
        // Note, currently this is not part of Elm syntax, i am just making my life easier by
        // adding a comma between multiple decals.
        let expr = elm::ExprParser::new()
            .parse("let x = 1, y = 1 in x")
            .unwrap();
        assert_eq!(&format!("{:?}", expr), "let \"x\" = 1 \"y\" = 1 in \"x\"");
    }

    #[test]
    fn test_lift() {
        assert!(elm::ExprParser::new().parse("lift1 (\\ x -> 1) x").is_ok());

        assert!(elm::ExprParser::new()
            .parse("lift2 (\\ x y -> 1) 1 2")
            .is_ok());

        let expr = elm::ExprParser::new()
            .parse("lift3 (\\ x y z -> 1) 1 2 3")
            .unwrap();
        assert_eq!(
            &format!("{:?}", expr),
            "((((\"lift3\" (\\ \"x\" \"y\" \"z\" -> 1)) 1) 2) 3)"
        );
    }
}
