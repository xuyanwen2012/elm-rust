use im::HashMap;

use rustelm_parser::ast::{Atom, Expr};

use crate::{
    error::{TypeCheckError, TypeCheckErrorType},
    Types,
};

type Context = im::HashMap<String, Types>;

/// The main entry to do typechecking. We type checking on root, and then recursively type
/// checking children.
pub fn typecheck_root(root: Box<Expr>) -> Result<Types, TypeCheckError> {
    let env = Context::new();

    typecheck(&env, root)
}

fn get_type_from_ctx(env: &Context, name: String) -> Result<Types, TypeCheckError> {
    match env.get(name.as_str()) {
        None => Err(TypeCheckError(TypeCheckErrorType::UndefinedName)),
        Some(ty) => Ok(ty.clone()),
    }
}

fn typecheck(env: &Context, term: Box<Expr>) -> Result<Types, TypeCheckError> {
    use Types::*;
    match *term {
        Expr::Const(atom) => match atom {
            Atom::Unit => Ok(Unit),
            Atom::Num(_) => Ok(Int),
            Atom::Var(var) => get_type_from_ctx(env, var),
        },
        Expr::Abs(args, expr) => {
            // Add bindings to context
            // TODO: Fix this
            Ok(Unit)
        }
        Expr::App(e1, e2) => {
            let type_e1 = typecheck(env, e1)?;
            let type_e2 = typecheck(env, e2)?;

            match type_e1 {
                Abs(t11, t12) => {
                    if type_e2 == *t11 {
                        Ok(*t12)
                    } else {
                        Err(TypeCheckError(TypeCheckErrorType::TypeMissMatch))
                    }
                }
                _ => Err(TypeCheckError(TypeCheckErrorType::TypeMissMatch)),
            }
        }
        Expr::BinOp(l, _, r) => {
            if Int == typecheck(env, r)? && Int == typecheck(env, l)? {
                Ok(Int)
            } else {
                Err(TypeCheckError(TypeCheckErrorType::TypeMissMatch))
            }
        }
        Expr::If(pred, e1, e2) => {
            if Int == typecheck(env, pred)? {
                let type_e1 = typecheck(env, e1)?;
                if type_e1 == typecheck(env, e2)? {
                    Ok(type_e1)
                } else {
                    Err(TypeCheckError(TypeCheckErrorType::TypeMissMatch))
                }
            } else {
                Err(TypeCheckError(TypeCheckErrorType::TypeMissMatch))
            }
        }
        Expr::Let(bindings, expr) => {
            // Add bindings
            let mut new_env = env.clone();

            for (name, e) in bindings {
                let type_e = typecheck(env, e)?;
                new_env.insert(name, type_e);
            }

            typecheck(new_env.as_ref(), expr)
        }
        Expr::Signal(_) => Ok(Unit),
        Expr::Lift(_, _) => Ok(Unit),
        Expr::Foldp(_, _, _) => Ok(Unit),
    }
}

mod test {
    use im::HashMap;

    use rustelm_parser::parser::parse;

    use crate::error::TypeCheckError;
    use crate::typechecker::{typecheck, typecheck_root, Context};

    use super::Types;

    #[test]
    fn test_hashmap() {
        let env = hashmap! { "x".to_owned() => Types::Unit };
        assert_eq!(&format!("{:?}", env), "{\"x\": Unit}");

        let mut new_env = env.clone();
        new_env.insert(String::from("y"), Types::Unit);

        assert!(new_env.contains_key("x"));
        assert!(new_env.contains_key("y"));
    }

    #[test]
    fn test_atom() {
        assert!(typecheck_root(parse("1").unwrap()).is_ok());
        assert!(typecheck_root(parse("()").unwrap()).is_ok());
        assert!(typecheck_root(parse("x").unwrap()).is_err());

        let fake_env = hashmap! { "x".to_owned() => Types::Int };
        assert!(typecheck(&fake_env, parse("x").unwrap()).is_ok());
        assert!(typecheck(&fake_env, parse("y").unwrap()).is_err());
    }

    #[test]
    fn test_binop() {
        assert!(typecheck_root(parse("1 + 1").unwrap()).is_ok());
        assert!(typecheck_root(parse("1 + ()").unwrap()).is_err());

        let fake_env = hashmap! { "x".to_owned() => Types::Int };
        assert!(typecheck(&fake_env, parse("x + x + 1").unwrap()).is_ok());
    }

    #[test]
    fn test_if() {
        match typecheck_root(parse("if 1 then 1 else 1").unwrap()) {
            Ok(ty) => assert_eq!(ty, Types::Int),
            Err(_) => assert! {false},
        }
        assert!(typecheck_root(parse("if 1 then () else ()").unwrap()).is_ok());
        assert!(typecheck_root(parse("if () then () else ()").unwrap()).is_err());
    }

    #[test]
    fn test_let() {
        assert!(typecheck_root(parse("let x = 1 in x").unwrap()).is_ok());
        assert!(typecheck_root(parse("let x = 1 in y").unwrap()).is_err());

        match typecheck_root(parse("let x = 1 + 2 in x").unwrap()) {
            Ok(result) => assert_eq!(result, Types::Int),
            Err(_) => assert! {false},
        }

        match typecheck_root(parse("let x = 1, y = 1, z = () in z").unwrap()) {
            Ok(result) => assert_eq!(result, Types::Unit),
            Err(_) => assert! {false},
        }

        match typecheck_root(parse("let x = 1 in let y = 1 in let z = 1 in x + y + z").unwrap()) {
            Ok(result) => assert_eq!(result, Types::Int),
            Err(_) => assert! {false},
        }
    }
}
