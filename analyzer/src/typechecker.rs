use crate::error::{TypeCheckError, TypeCheckErrorType};
use rustelm_parser::ast::SimpleType::{Int, Unit};
use rustelm_parser::{
    ast,
    ast::SignalType::{Abs1, Abs2},
    ast::{Atom, Expr, SignalType, SimpleType, Types},
};

type Context = im::HashMap<String, ast::Types>;

// Some Elm input signals and signal constructors
lazy_static! {
    static ref INPUTS: Context = {
        use SignalType::*;
        use SimpleType::*;
        im::hashmap! {
            "MouseX".to_owned() => Types::Signal(Signal(Int)),
            "MouseY".to_owned() => Types::Signal(Signal(Int)),
            "MouseClicks".to_owned() => Types::Signal(Signal(Unit)),
        }
    };
}

/// The main entry to do typechecking. We type checking on root, and then recursively type
/// checking children.
pub fn typecheck_root(root: Box<ast::Expr>) -> Result<ast::Types, TypeCheckError> {
    // let env: Context = INPUTS.clone();
    get_type_of(&INPUTS, root)
}

fn get_type_from_ctx(env: &Context, name: String) -> Result<ast::Types, TypeCheckError> {
    match env.get(name.as_str()) {
        None => Err(TypeCheckError(TypeCheckErrorType::UndefinedName)),
        Some(ty) => Ok(ty.clone()),
    }
}

fn get_type_of(env: &Context, term: Box<ast::Expr>) -> Result<ast::Types, TypeCheckError> {
    use ast::SimpleType::*;
    use ast::Types::*;

    match *term {
        Expr::Const(atom) => match atom {
            Atom::Unit => Ok(Simple(Unit)),
            Atom::Num(_) => Ok(Simple(Int)),
            Atom::Var(name) | Atom::Signal(name) => get_type_from_ctx(env, name),
        },
        Expr::Abs(atom, param_ty, expr) => match atom {
            Atom::Var(name) | Atom::Signal(name) => {
                // Add the new binding to the environment, then get the type of the expression in
                // the new environment.
                let mut new_env = env.clone();
                new_env.insert(name, param_ty.clone());
                let return_ty = get_type_of(new_env.as_ref(), expr)?;

                // We need to manually check the lambda creates a "o -> t" type.
                match return_ty {
                    Simple(sim_ty) => match param_ty {
                        // t -> t'
                        Simple(sim_ty0) => Ok(Simple(Abs(Box::new(sim_ty0), Box::new(sim_ty)))),
                        // o -> t, which should be prohibited
                        Signal(_) => Err(TypeCheckError(TypeCheckErrorType::InvalidParamType)),
                    },
                    Signal(sig_ty) => match param_ty {
                        // t -> o
                        Simple(sim_ty0) => Ok(Signal(Abs1(sim_ty0, Box::new(sig_ty)))),
                        // o -> o
                        Signal(sig_ty0) => Ok(Signal(Abs2(Box::new(sig_ty0), Box::new(sig_ty)))),
                    },
                }
            }
            _ => Err(TypeCheckError(TypeCheckErrorType::ExpectIdentifier)),
        },
        Expr::App(e1, e2) => {
            let arg_ty = get_type_of(env, e2)?;

            match get_type_of(env, e1)? {
                Simple(ty) => match ty {
                    Abs(sim_ty, ty2) => {
                        if Simple(*sim_ty) == arg_ty {
                            Ok(Simple(*ty2))
                        } else {
                            Err(TypeCheckError(TypeCheckErrorType::TypeMissMatch))
                        }
                    }
                    _ => Err(TypeCheckError(TypeCheckErrorType::InvalidParamType)),
                },
                Signal(ty) => match ty {
                    Abs1(sim_ty, sig_ty) => {
                        if Simple(sim_ty) == arg_ty {
                            Ok(Signal(*sig_ty))
                        } else {
                            Err(TypeCheckError(TypeCheckErrorType::TypeMissMatch))
                        }
                    }
                    Abs2(sig_ty1, sig_ty2) => {
                        if Signal(*sig_ty1) == arg_ty {
                            Ok(Signal(*sig_ty2))
                        } else {
                            Err(TypeCheckError(TypeCheckErrorType::TypeMissMatch))
                        }
                    }
                    _ => Err(TypeCheckError(TypeCheckErrorType::InvalidParamType)),
                },
            }
        }
        Expr::BinOp(e1, _, e2) => {
            if Simple(Int) == get_type_of(env, e1)? && Simple(Int) == get_type_of(env, e2)? {
                Ok(Simple(Int))
            } else {
                Err(TypeCheckError(TypeCheckErrorType::TypeMissMatch))
            }
        }
        Expr::If(e1, e2, e3) => {
            if Simple(Int) == get_type_of(env, e1)? {
                let ty = get_type_of(env, e2)?;
                if ty == get_type_of(env, e3)? {
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
                // Add the new binding to the environment, then get the type of the expression in
                // the new environment.
                let mut new_env = env.clone();
                new_env.insert(name, get_type_of(env, e1)?);
                get_type_of(new_env.as_ref(), e2)
            }
            _ => Err(TypeCheckError(TypeCheckErrorType::ExpectIdentifier)),
        },
        // TODO: Fix this part
        Expr::Lift(n, expr, vec) => {
            // First we construct a vector of all the argument types
            let mut types = vec![];
            for atom in vec {
                let ty_i = match atom {
                    Atom::Var(input) => get_type_from_ctx(env, input),
                    _ => Err(TypeCheckError(TypeCheckErrorType::ExpectIdentifier)),
                }?;

                // and make sure it is simple type
                match ty_i {
                    Simple(_) => unreachable!(),
                    Signal(sig_ty) => match sig_ty {
                        SignalType::Signal(s) => types.push(s),
                        _ => unreachable!(),
                    },
                }
            }

            let ty = get_type_of(env, expr)?;

            let mut lift_ty = vec![];
            match ty.clone() {
                Simple(x) => {
                    foo(lift_ty.as_mut(), x.clone());
                }
                _ => unreachable!(),
            };

            // In current implementation, the last element is the return type. Thus we can
            // Simply compare the list except the last one
            let return_ty = lift_ty.remove(lift_ty.len() - 1);
            if lift_ty == types {
                Ok(Simple(return_ty))
            } else {
                Err(TypeCheckError(TypeCheckErrorType::TypeMissMatch))
            }
        }
        Expr::Foldp(_, _, _) => unimplemented!(),
    }
}

fn foo(vec: &mut Vec<SimpleType>, ty: SimpleType) {
    match ty {
        SimpleType::Unit => vec.push(Unit),
        SimpleType::Int => vec.push(Int),
        SimpleType::Abs(l, r) => {
            foo(vec, *l);
            foo(vec, *r);
        }
    }
}

#[cfg(test)]
mod test {
    use super::{get_type_of, typecheck_root};
    use rustelm_parser::{
        ast::{
            SignalType,
            SimpleType::{Abs, Int, Unit},
            Types::*,
        },
        parser::parse,
    };

    #[test]
    fn test_atom() {
        assert_eq!(typecheck_root(parse("1\n").unwrap()).unwrap(), Simple(Int));
        assert_eq!(
            typecheck_root(parse("()\n").unwrap()).unwrap(),
            Simple(Unit)
        );
        assert!(typecheck_root(parse("x\n").unwrap()).is_err());

        let fake_env = im::hashmap! { "x".to_owned() => Simple(Int) };
        assert!(get_type_of(&fake_env, parse("y\n").unwrap()).is_err());
        assert_eq!(
            get_type_of(&fake_env, parse("x\n").unwrap()).unwrap(),
            Simple(Int)
        );
    }

    #[test]
    fn test_signal() {
        assert_eq!(
            typecheck_root(parse("MouseClicks\n").unwrap()).unwrap(),
            Signal(SignalType::Signal(Unit))
        );

        assert_eq!(
            typecheck_root(parse("MousePosition\n").unwrap()).unwrap(),
            Signal(SignalType::Signal(Int))
        );
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

        // Signal abs
        assert_eq!(
            &format!(
                "{:?}",
                typecheck_root(parse("\\x: int. MouseClicks\n").unwrap()).unwrap()
            ),
            "(int -> sig(unit))"
        );

        assert_eq!(
            &format!(
                "{:?}",
                typecheck_root(parse("\\x: signal unit.. MouseClicks\n").unwrap()).unwrap()
            ),
            "(sig(unit) -> sig(unit))"
        );

        assert!(typecheck_root(parse("\\x: signal unit.. 1\n").unwrap()).is_err());
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

        // Because it is missing the last argument thus it return type (int -> int)
        assert_eq!(
            typecheck_root(parse("(\\x: int. \\y: int. \\z: int. x + y + z) 1 2\n").unwrap())
                .unwrap(),
            Simple(Abs(Box::new(Int), Box::new(Int)))
        );

        // Signal type
        assert_eq!(
            typecheck_root(parse("(\\x: signal int.. x) MousePosition\n").unwrap()).unwrap(),
            Signal(SignalType::Signal(Int))
        );

        assert!(typecheck_root(parse("(\\x: signal int.. x) 1\n").unwrap()).is_err(),);
        assert!(typecheck_root(parse("(\\x: signal int.. x) MouseClicks\n").unwrap()).is_err(),);
    }

    #[test]
    fn test_binop() {
        assert!(typecheck_root(parse("1 + 1\n").unwrap()).is_ok());
        assert!(typecheck_root(parse("1 + ()\n").unwrap()).is_err());

        let fake_env = im::hashmap! { "x".to_owned() => Simple(Int) };
        assert!(get_type_of(&fake_env, parse("x + x + 1\n").unwrap()).is_ok());
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

    #[test]
    fn test_lift() {
        assert!(typecheck_root(parse("lift1 (\\ x: unit. 1): MouseClicks\n").unwrap()).is_ok());

        assert!(
            typecheck_root(parse("lift2 (\\ x: int. \\y: int. ()): MouseX MouseY\n").unwrap())
                .is_ok()
        );

        assert!(typecheck_root(
            parse("lift2 (\\ x: int. \\y: int. ()): MouseClicks MouseClicks\n").unwrap()
        )
        .is_err());
    }
}
