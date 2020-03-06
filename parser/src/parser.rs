// use lalrpop_util::ParseError as LalrParseError;

use crate::{ast, elm, lexer};

#[derive(Debug)]
pub enum ParserError {
    LalrError,
}

pub fn parse(input: &str) -> Result<Box<ast::Expr>, ParserError> {
    assert!(input.ends_with('\n'));

    let lxr = lexer::Lexer::new(input);
    match elm::ExprParser::new().parse(lxr) {
        Err(_) => Err(ParserError::LalrError),
        Ok(value) => Ok(value),
    }
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn test_constant() {
        assert!(parse("42\n").is_ok());
        assert!(parse("()\n").is_ok());
        assert!(parse("x\n").is_ok());
        assert!(parse("())\n").is_err());
        assert!(parse("'\n").is_err());
    }

    #[test]
    fn test_lambda() {
        assert!(parse("\\1: int -> x\n").is_err());
        assert!(parse("\\x -> x\n").is_err());
        assert!(parse("\\ -> x\n").is_err());

        // Simple Types
        let expr = parse("\\x: unit. x\n").unwrap();
        assert_eq!(&format!("{:?}", expr), "\\\"x\": unit. -> \"x\"");

        let expr = parse("\\x: int -> int -> int. x\n").unwrap();
        assert_eq!(
            &format!("{:?}", expr),
            "\\\"x\": ((int -> int) -> int). -> \"x\""
        );

        let expr = parse("\\x: int -> (int -> int). x\n").unwrap();
        assert_eq!(
            &format!("{:?}", expr),
            "\\\"x\": (int -> (int -> int)). -> \"x\""
        );

        // Signal Types
        let expr = parse("\\x: signal unit.. x\n").unwrap();
        assert_eq!(&format!("{:?}", expr), "\\\"x\": signal unit.. -> \"x\"");

        let expr = parse("\\x: int -> signal unit.. x\n").unwrap();
        assert_eq!(
            &format!("{:?}", expr),
            "\\\"x\": signal (int -> unit).. -> \"x\""
        );
    }

    #[test]
    fn test_app() {
        assert!(parse("1 1\n").is_ok());

        let expr = parse("() ()\n").unwrap();
        assert_eq!(&format!("{:?}", expr), "(() ())");

        // Note: this one should be checked by type checker
        let expr = parse("(\\ x: int. 1) 1\n").unwrap();
        assert_eq!(&format!("{:?}", expr), "(\\\"x\": int. -> 1 1)");
    }

    #[test]
    fn test_if() {
        let expr = parse("if 1 then 2 else 3\n").unwrap();
        assert_eq!(&format!("{:?}", expr), "if ( 1 ) then { 2 } else { 3 }");

        let expr = parse("if (if 1 then 2 else 3) then 2 else 3\n").unwrap();
        assert_eq!(
            &format!("{:?}", expr),
            "if ( if ( 1 ) then { 2 } else { 3 } ) then { 2 } else { 3 }"
        );
    }

    #[test]
    fn test_binop() {
        // Literals
        let expr = parse("1 + 2 + 3\n").unwrap();
        assert_eq!(&format!("{:?}", expr), "((1 + 2) + 3)");

        // Literal with Identifiers
        let expr = parse("a + b * 3\n").unwrap();
        assert_eq!(&format!("{:?}", expr), "(\"a\" + (\"b\" * 3))");

        // Compare
        let expr = parse("a == b\n").unwrap();
        assert_eq!(&format!("{:?}", expr), "(\"a\" == \"b\")");

        let expr = parse("1 + 2 * 3 == 3 * 2 + 1\n").unwrap();
        assert_eq!(&format!("{:?}", expr), "((1 + (2 * 3)) == ((3 * 2) + 1))")
    }

    #[test]
    fn test_let() {
        let expr = parse("let x = 1 + 2 in x\n").unwrap();
        assert_eq!(&format!("{:?}", expr), "let \"x\" = (1 + 2) in \"x\"");

        let expr = parse("let x = 1 in let y = 2 in let z = 3 in x + y + z\n").unwrap();
        assert_eq!(
            &format!("{:?}", expr),
            "let \"x\" = 1 in let \"y\" = 2 in let \"z\" = 3 in ((\"x\" + \"y\") + \"z\")"
        );

        assert!(parse("let 1 = 1 + 2 in x\n").is_err());
        assert!(parse("let 1 = 1 + 2\n").is_err());
    }

    #[test]
    fn test_lift() {
        assert!(parse("lift1 (\\ x: int. 1) x\n").is_ok());

        let expr = parse("lift3 (\\ x:int. \\y:int. \\z:int. 1) 1 2 3\n").unwrap();
        assert_eq!(
            &format!("{:?}", expr),
            "((((\"lift3\" \\\"x\": int. -> \\\"y\": int. -> \\\"z\": int. -> 1) 1) 2) 3)"
        );
    }
}
