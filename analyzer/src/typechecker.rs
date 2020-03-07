use crate::error::{TypeCheckError, TypeCheckErrorType};
use im::HashMap;
use rustelm_parser::ast;
use rustelm_parser::ast::{Atom, Expr, SimpleType, Types};
use std::process::exit;

type Context = im::HashMap<String, ast::Types>;

/// The main entry to do typechecking. We type checking on root, and then recursively type
/// checking children.
pub fn typecheck_root(root: Box<ast::Expr>) -> Result<ast::Types, TypeCheckError> {
    let env = Context::new();

    typecheck(&env, root)
}

fn get_type_from_ctx(env: &Context, name: String) -> Result<ast::Types, TypeCheckError> {
    match env.get(name.as_str()) {
        None => Err(TypeCheckError(TypeCheckErrorType::UndefinedName)),
        Some(ty) => Ok(ty.clone()),
    }
}

fn typecheck(env: &Context, term: Box<ast::Expr>) -> Result<ast::Types, TypeCheckError> {
    use ast::SimpleType::*;
    use ast::Types::*;

    match *term {
        Expr::Const(atom) => match atom {
            Atom::Unit => Ok(Simple(Unit)),
            Atom::Num(_) => Ok(Simple(Int)),
            Atom::Var(name) => get_type_from_ctx(env, name),
        },
        Expr::Abs(atom, ty, expr) => match atom {
            Atom::Var(name) => {
                let mut new_env = env.clone();
                new_env.insert(name, ty.clone());
                let ty2 = typecheck(new_env.as_ref(), expr)?;

                match ty2 {
                    Simple(sim_ty) => match ty {
                        Simple(sim_ty0) => Ok(Simple(Abs(Box::new(sim_ty0), Box::new(sim_ty)))),
                        Signal(_) => unimplemented!(),
                    },
                    Signal(_) => unimplemented!(),
                }
            }
            _ => Err(TypeCheckError(TypeCheckErrorType::ExpectIdentifier)),
        },
        Expr::App(e1, e2) => {
            let arg_ty = typecheck(env, e2)?;

            match typecheck(env, e1)? {
                Simple(ty) => match ty {
                    Abs(sim_ty, ty2) => {
                        if Simple(*sim_ty) == arg_ty {
                            Ok(Simple(*ty2))
                        } else {
                            Err(TypeCheckError(TypeCheckErrorType::ExpectIdentifier))
                        }
                    }
                    _ => Err(TypeCheckError(TypeCheckErrorType::ExpectIdentifier)),
                },
                Signal(_) => unimplemented!(),
            }
        }
        Expr::BinOp(e1, _, e2) => {
            if Simple(Int) == typecheck(env, e1)? && Simple(Int) == typecheck(env, e2)? {
                Ok(Simple(Int))
            } else {
                Err(TypeCheckError(TypeCheckErrorType::TypeMissMatch))
            }
        }
        Expr::If(e1, e2, e3) => {
            if Simple(Int) == typecheck(env, e1)? {
                let ty = typecheck(env, e2)?;
                if ty == typecheck(env, e3)? {
                    Ok(ty)
                } else {
                    Err(TypeCheckError(TypeCheckErrorType::TypeMissMatch))
                }
            } else {
                Err(TypeCheckError(TypeCheckErrorType::TypeMissMatch))
            }
        }
        Expr::Let(atom, e1, e2) => match atom {
            Atom::Var(name) => {
                let mut new_env = env.clone();
                new_env.insert(name, typecheck(env, e1)?);
                typecheck(new_env.as_ref(), e2)
            }
            _ => Err(TypeCheckError(TypeCheckErrorType::TypeMissMatch)),
        },
        Expr::Signal(ref i) => Err(TypeCheckError(TypeCheckErrorType::UndefinedName)),
        Expr::Lift(_, _) => Err(TypeCheckError(TypeCheckErrorType::UndefinedName)),
        Expr::Foldp(_, _, _) => Err(TypeCheckError(TypeCheckErrorType::UndefinedName)),
    }
}

mod test {
    use crate::error::TypeCheckError;
    use crate::typechecker::{typecheck, typecheck_root, Context};
    use im::HashMap;
    use rustelm_parser::ast::SimpleType::{Abs, Int, Unit};
    use rustelm_parser::ast::Types::Simple;
    use rustelm_parser::parser::parse;
    use rustelm_parser::tokens::Token::In;

    #[test]
    fn test_hashmap() {
        let env = hashmap! { "x".to_owned() => Simple(Unit) };
        assert_eq!(&format!("{:?}", env), "{\"x\": unit}");

        let mut new_env = env.clone();
        new_env.insert(String::from("y"), Simple(Unit));

        assert!(new_env.contains_key("x"));
        assert!(new_env.contains_key("y"));
    }

    #[test]
    fn test_atom() {
        let ty = typecheck_root(parse("1\n").unwrap()).unwrap();
        assert_eq!(&format!("{:?}", ty), "int");

        let ty = typecheck_root(parse("()\n").unwrap()).unwrap();
        assert_eq!(&format!("{:?}", ty), "unit");

        assert!(typecheck_root(parse("x\n").unwrap()).is_err());

        let fake_env = hashmap! { "x".to_owned() => Simple(Int) };

        let ty = typecheck(&fake_env, parse("x\n").unwrap()).unwrap();
        assert_eq!(&format!("{:?}", ty), "int");

        assert!(typecheck(&fake_env, parse("y\n").unwrap()).is_err());
    }

    #[test]
    fn test_abs() {
        assert_eq!(
            &format!(
                "{:?}",
                typecheck_root(parse("\\x: int. x\n").unwrap()).unwrap()
            ),
            "(int -> int)"
        );

        assert_eq!(
            &format!(
                "{:?}",
                typecheck_root(parse("\\x: int. \\y: int. \\z: int. x + y + z\n").unwrap())
                    .unwrap()
            ),
            "(int -> (int -> (int -> int)))"
        );
    }

    #[test]
    fn test_app() {
        assert!(typecheck_root(parse("(\\x: int. x) 1\n").unwrap()).is_ok());
        assert!(typecheck_root(parse("(\\x: int. x) ()\n").unwrap()).is_err());
        assert!(typecheck_root(parse("(\\x: unit. x) 1\n").unwrap()).is_err());
        assert!(typecheck_root(parse("(\\x: unit. x) ()\n").unwrap()).is_ok());

        assert_eq!(
            typecheck_root(parse("(\\x: int. \\y: int. \\z: int. x + y + z) 1 2 3\n").unwrap())
                .unwrap(),
            Simple(Int)
        );

        assert_eq!(
            typecheck_root(parse("(\\x: int. \\y: int. \\z: int. x + y + z) 1 2\n").unwrap())
                .unwrap(),
            Simple(Abs(Box::new(Int), Box::new(Int)))
        );
    }

    #[test]
    fn test_binop() {
        assert!(typecheck_root(parse("1 + 1\n").unwrap()).is_ok());
        assert!(typecheck_root(parse("1 + ()\n").unwrap()).is_err());

        let fake_env = hashmap! { "x".to_owned() => Simple(Int) };
        assert!(typecheck(&fake_env, parse("x + x + 1\n").unwrap()).is_ok());
    }

    #[test]
    fn test_if() {
        assert_eq!(
            typecheck_root(parse("if 1 then 1 else 1\n").unwrap()).unwrap(),
            Simple(Int)
        );

        assert!(typecheck_root(parse("if 1 then () else ()\n").unwrap()).is_ok());
        assert!(typecheck_root(parse("if () then () else ()\n").unwrap()).is_err());
    }

    #[test]
    fn test_let() {
        assert!(typecheck_root(parse("let x = 1 in x\n").unwrap()).is_ok());
        assert!(typecheck_root(parse("let x = 1 in y\n").unwrap()).is_err());

        assert_eq!(
            typecheck_root(parse("let x = 1 + 2 in x\n").unwrap()).unwrap(),
            Simple(Int)
        );

        assert_eq!(
            typecheck_root(parse("let x = 1 in let y = 1 in let z = 1 in x + y + z\n").unwrap())
                .unwrap(),
            Simple(Int),
        );
    }
}
