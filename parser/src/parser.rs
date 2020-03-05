use lalrpop_util::ParseError as LalrParseError;

use crate::{ast, elm, lexer};

#[derive(Debug)]
pub enum ParserError {
    TBD,
}

pub fn parse(input: &str) -> Result<Box<ast::Expr>, ParserError> {
    assert!(input.ends_with('\n'));

    let lxr = lexer::Lexer::new(input);
    match elm::ExprParser::new().parse(lxr) {
        Err(err) => Err(ParserError::TBD),
        Ok(value) => Ok(value),
    }
}

#[cfg(test)]
mod tests {
    use super::parse;
    use crate::ast;
    use num_bigint::BigInt;

    #[test]
    fn test_constant() {
        assert!(parse("42\n").is_ok());
        assert!(parse("()\n").is_ok());
        assert!(parse("x\n").is_ok());
        assert!(parse("())\n").is_err());
        assert!(parse("'\n").is_err());
    }

    #[test]
    fn test_types() {
        assert!(parse("\\1: int -> x\n").is_err());
        assert!(parse("\\x -> x\n").is_err());
        assert!(parse("\\ -> x\n").is_err());

        let expr = parse("\\x: unit -> x\n").unwrap();
        assert_eq!(&format!("{:?}", expr), "\\\"x\": unit. -> \"x\"");

        let expr = parse("\\x: int -> int -> int -> x\n").unwrap();
        assert_eq!(
            &format!("{:?}", expr),
            "\\\"x\": int -> int -> int. -> \"x\""
        );
    }

    //
    // #[test]
    // fn test_abs() {
    //     assert!(parse("\\ -> 1").is_err());
    //     assert!(parse("\\ 1 -> 1").is_err());
    //     assert!(parse("1 -> 1").is_err());
    //
    //     let expr = parse("\\ x y z -> ()").unwrap();
    //     assert_eq!(&format!("{:?}", expr), "(\\ \"x\" \"y\" \"z\" -> ())");
    // }
    //
    // #[test]
    // fn test_app() {
    //     assert!(parse("1 1").is_ok());
    //
    //     let expr = parse("() ()").unwrap();
    //     assert_eq!(&format!("{:?}", expr), "(() ())");
    //
    //     // Note: this one should be checked by type checker
    //     let expr = parse("\\ x -> 1 1").unwrap();
    //     assert_eq!(&format!("{:?}", expr), "(\\ \"x\" -> (1 1))");
    // }
    //
    // #[test]
    // fn test_binop() {
    //     // Literals
    //     let expr = parse("1 + 2 * 3").unwrap();
    //     assert_eq!(&format!("{:?}", expr), "(1 + (2 * 3))");
    //
    //     // Literal with Identifiers
    //     let expr = parse("a + b * 3").unwrap();
    //     assert_eq!(&format!("{:?}", expr), "(\"a\" + (\"b\" * 3))");
    //
    //     // Compare
    //     let expr = parse("a == b").unwrap();
    //     assert_eq!(&format!("{:?}", expr), "(\"a\" == \"b\")");
    //
    //     let expr = parse("1 + 2 * 3 == 3 * 2 + 1").unwrap();
    //     assert_eq!(&format!("{:?}", expr), "((1 + (2 * 3)) == ((3 * 2) + 1))")
    // }
    //
    // #[test]
    // fn test_if() {
    //     // Simple if else
    //     let expr = parse("if 1 then 2 else 3").unwrap();
    //     assert_eq!(&format!("{:?}", expr), "if ( 1 ) then { 2 } else { 3 }");
    //
    //     // Complex if else
    //     let expr = parse("if 1 + 2 then 2 else 3").unwrap();
    //     assert_eq!(
    //         &format!("{:?}", expr),
    //         "if ( (1 + 2) ) then { 2 } else { 3 }"
    //     );
    //
    //     // More Complex if else
    //     let expr = parse("if 1 + 2 then 2 - 3 else 3 + 4 + 5").unwrap();
    //     assert_eq!(
    //         &format!("{:?}", expr),
    //         "if ( (1 + 2) ) then { (2 - 3) } else { ((3 + 4) + 5) }"
    //     );
    //
    //     // Very tricky pred
    //     let expr = parse("if (if 1 then 2 else 3) then 2 else 3").unwrap();
    //     assert_eq!(
    //         &format!("{:?}", expr),
    //         "if ( if ( 1 ) then { 2 } else { 3 } ) then { 2 } else { 3 }"
    //     );
    //
    //     // With Identifiers
    //     let expr = parse("if powerLevel > 9000 then 1111 else 2222").unwrap();
    //     assert_eq!(
    //         &format!("{:?}", expr),
    //         "if ( (\"powerLevel\" > 9000) ) then { 1111 } else { 2222 }"
    //     );
    //
    //     // If-else-if
    //     let expr = parse(
    //         "\
    //               if key == 40 then
    //                   n + 1
    //
    //               else if key == 38 then
    //                   n - 1
    //
    //               else
    //                   n",
    //     )
    //     .unwrap();
    //     assert_eq!(
    //         &format!("{:?}", expr),
    //         "if ( (\"key\" == 40) ) then { (\"n\" + 1) } else { if ( (\"key\" == 38) ) then { (\"n\" - 1) } else { \"n\" } }"
    //     );
    // }
    //
    // #[test]
    // fn test_let() {
    //     // Single
    //     let expr = parse("let x = 1 + 2 in x").unwrap();
    //     assert_eq!(&format!("{:?}", expr), "let \"x\" = (1 + 2) in \"x\"");
    //
    //     assert!(parse("let 1 = 1 + 2 in x").is_err());
    //     assert!(parse("let 1 = 1 + 2").is_err());
    //
    //     // Multiple
    //     // Note, currently this is not part of Elm syntax, i am just making my life easier by
    //     // adding a comma between multiple decals.
    //     let expr = parse("let x = 1, y = 1 in x").unwrap();
    //     assert_eq!(&format!("{:?}", expr), "let \"x\" = 1 \"y\" = 1 in \"x\"");
    // }
    //
    // #[test]
    // fn test_lift() {
    //     assert!(parse("lift1 (\\ x -> 1) x").is_ok());
    //
    //     assert!(parse("lift2 (\\ x y -> 1) 1 2").is_ok());
    //
    //     let expr = parse("lift3 (\\ x y z -> 1) 1 2 3").unwrap();
    //     assert_eq!(
    //         &format!("{:?}", expr),
    //         "((((\"lift3\" (\\ \"x\" \"y\" \"z\" -> 1)) 1) 2) 3)"
    //     );
    // }
}
