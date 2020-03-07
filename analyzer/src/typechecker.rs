use crate::error::{TypeCheckError, TypeCheckErrorType};
use im::HashMap;
use rustelm_parser::ast;
use rustelm_parser::ast::{Atom, Expr};
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
                new_env.insert(name, ty);
                typecheck(new_env.as_ref(), expr)
            }
            _ => Err(TypeCheckError(TypeCheckErrorType::ExpectIdentifier)),
        },
        Expr::App(ref e1, ref e2) => Err(TypeCheckError(TypeCheckErrorType::UndefinedName)),
        Expr::BinOp(e1, _, e2) => {
            if Simple(Int) == typecheck(env, e1)? && Simple(Int) == typecheck(env, e2)? {
                Ok(Simple(Int))
            } else {
                Err(TypeCheckError(TypeCheckErrorType::TypeMissMatch))
            }
        }
        Expr::If(e1, e2, e3) => {
            if Simple(Int) == typecheck(env, e1)? {
                let ty1 = typecheck(env, e2)?;
                let ty2 = typecheck(env, e3)?;
                println!("{:?}", ty2);

                if ty1 == ty2 {
                    Ok(ty1)
                } else {
                    Err(TypeCheckError(TypeCheckErrorType::TypeMissMatch))
                }
            } else {
                Err(TypeCheckError(TypeCheckErrorType::TypeMissMatch))
            }
        }
        Expr::Let(ref atom, ref e1, ref e2) => {
            Err(TypeCheckError(TypeCheckErrorType::UndefinedName))
        }
        Expr::Signal(ref i) => Err(TypeCheckError(TypeCheckErrorType::UndefinedName)),
        Expr::Lift(_, _) => Err(TypeCheckError(TypeCheckErrorType::UndefinedName)),
        Expr::Foldp(_, _, _) => Err(TypeCheckError(TypeCheckErrorType::UndefinedName)),
    }
}

mod test {
    use crate::error::TypeCheckError;
    use crate::typechecker::{typecheck, typecheck_root, Context};
    use im::HashMap;
    use rustelm_parser::ast::SimpleType::{Int, Unit};
    use rustelm_parser::ast::Types::Simple;
    use rustelm_parser::parser::parse;

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
    fn test_binop() {
        assert!(typecheck_root(parse("1 + 1\n").unwrap()).is_ok());
        assert!(typecheck_root(parse("1 + ()\n").unwrap()).is_err());

        let fake_env = hashmap! { "x".to_owned() => Simple(Int) };
        assert!(typecheck(&fake_env, parse("x + x + 1\n").unwrap()).is_ok());
    }
    //
    // #[test]
    // fn test_if() {
    //     match typecheck_root(parse("if 1 then 1 else 1").unwrap()) {
    //         Ok(ty) => assert_eq!(ty, Types::Int),
    //         Err(_) => assert! {false},
    //     }
    //     assert!(typecheck_root(parse("if 1 then () else ()").unwrap()).is_ok());
    //     assert!(typecheck_root(parse("if () then () else ()").unwrap()).is_err());
    // }
    //
    // #[test]
    // fn test_let() {
    //     assert!(typecheck_root(parse("let x = 1 in x").unwrap()).is_ok());
    //     assert!(typecheck_root(parse("let x = 1 in y").unwrap()).is_err());
    //
    //     match typecheck_root(parse("let x = 1 + 2 in x").unwrap()) {
    //         Ok(result) => assert_eq!(result, Types::Int),
    //         Err(_) => assert! {false},
    //     }
    //
    //     match typecheck_root(parse("let x = 1, y = 1, z = () in z").unwrap()) {
    //         Ok(result) => assert_eq!(result, Types::Unit),
    //         Err(_) => assert! {false},
    //     }
    //
    //     match typecheck_root(parse("let x = 1 in let y = 1 in let z = 1 in x + y + z").unwrap()) {
    //         Ok(result) => assert_eq!(result, Types::Int),
    //         Err(_) => assert! {false},
    //     }
    // }
}
