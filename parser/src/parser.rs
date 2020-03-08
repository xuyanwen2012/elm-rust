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
        assert_eq!(
            &format!("{:?}", parse("\\x: unit. x\n").unwrap()),
            "\\\"x\": unit. -> \"x\""
        );

        assert_eq!(
            &format!("{:?}", parse("\\x: int -> int -> int. x\n").unwrap()),
            "\\\"x\": ((int -> int) -> int). -> \"x\""
        );

        assert_eq!(
            &format!("{:?}", parse("\\x: int -> (int -> int). x\n").unwrap()),
            "\\\"x\": (int -> (int -> int)). -> \"x\""
        );

        // Signal Types
        assert_eq!(
            &format!("{:?}", parse("\\x: signal unit.. x\n").unwrap()),
            "\\\"x\": signal unit.. -> \"x\""
        );

        assert_eq!(
            &format!("{:?}", parse("\\x: int -> signal unit.. x\n").unwrap()),
            "\\\"x\": (int -> sig(unit)). -> \"x\""
        );
    }

    #[test]
    fn test_app() {
        assert!(parse("1 1\n").is_ok());

        assert_eq!(&format!("{:?}", parse("() ()\n").unwrap()), "(() ())");

        // Note: this one should be checked by type checker
        assert_eq!(
            &format!("{:?}", parse("(\\ x: int. 1) 1\n").unwrap()),
            "(\\\"x\": int. -> 1 1)"
        );
    }

    #[test]
    fn test_if() {
        assert_eq!(
            &format!("{:?}", parse("if 1 then 2 else 3\n").unwrap()),
            "if ( 1 ) then { 2 } else { 3 }"
        );

        assert_eq!(
            &format!(
                "{:?}",
                parse("if (if 1 then 2 else 3) then 2 else 3\n").unwrap()
            ),
            "if ( if ( 1 ) then { 2 } else { 3 } ) then { 2 } else { 3 }"
        );
    }

    #[test]
    fn test_binop() {
        // Literals
        assert_eq!(
            &format!("{:?}", parse("1 + 2 + 3\n").unwrap()),
            "((1 + 2) + 3)"
        );

        // Literal with Identifiers
        assert_eq!(
            &format!("{:?}", parse("a + b * 3\n").unwrap()),
            "(\"a\" + (\"b\" * 3))"
        );

        // Compare
        assert_eq!(
            &format!("{:?}", parse("a == b\n").unwrap()),
            "(\"a\" == \"b\")"
        );

        assert_eq!(
            &format!("{:?}", parse("1 + 2 * 3 == 3 * 2 + 1\n").unwrap()),
            "((1 + (2 * 3)) == ((3 * 2) + 1))"
        )
    }

    #[test]
    fn test_let() {
        assert_eq!(
            &format!("{:?}", parse("let x = 1 + 2 in x\n").unwrap()),
            "let \"x\" = (1 + 2) in \"x\""
        );

        assert_eq!(
            &format!(
                "{:?}",
                parse("let x = 1 in let y = 2 in let z = 3 in x + y + z\n").unwrap()
            ),
            "let \"x\" = 1 in let \"y\" = 2 in let \"z\" = 3 in ((\"x\" + \"y\") + \"z\")"
        );

        assert!(parse("let 1 = 1 + 2 in x\n").is_err());
        assert!(parse("let 1 = 1 + 2\n").is_err());
    }

    #[test]
    fn test_lift() {
        assert!(parse("lift1 (\\ x: int. 1): MouseX\n").is_ok());

        assert_eq!(
            &format!(
                "{:?}",
                parse("lift2 (\\ x:int. \\y:int. ()): MouseX MouseY\n").unwrap()
            ),
            "lift2 \\\"x\": int. -> \\\"y\": int. -> () \"MouseX\" \"MouseY\"!"
        );
    }
}
