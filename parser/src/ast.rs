use std::fmt::{Debug, Error, Formatter};

pub enum Expr {
    Unit,
    Number(i32),
    Identifier(String),
    BinOp(Box<Expr>, Opcode, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Let(Vec<(String, Box<Expr>)>, Box<Expr>),
    //    Lambda,
    //    Application,
    Error,
}

#[derive(Copy, Clone)]
pub enum Opcode {
    // Arithmetic
    Mul,
    Div,
    Add,
    Sub,
    // Comparison
    Eq,
    Ne,
    Le,
    Ge,
    Less,
    Greater,
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(n) => write!(fmt, "{:?}", n),
            BinOp(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Error => write!(fmt, "error"),
            Unit => write!(fmt, "()"),
            If(ref pred, ref if_true, ref if_false) => write!(
                fmt,
                "if ( {:?} ) then {{ {:?} }} else {{ {:?} }}",
                pred, if_true, if_false
            ),
            Identifier(ref str) => write!(fmt, "{:?}", str),
            Let(ref vec, ref e2) => {
                write!(fmt, "let ");
                for (ref x, ref e1) in vec {
                    write!(fmt, "{:?} = {:?}, ", x, e1)
                }
                write!(" in {:?}", e2)
            } // Let(ref x, ref e1, ref e2) => write!(fmt, "let {:?} = {:?} in {:?}", x, e1, e2),
        }
    }
}

impl Debug for Opcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Opcode::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
            Eq => write!(fmt, "=="),
            Ne => write!(fmt, "!="),
            Le => write!(fmt, "<="),
            Ge => write!(fmt, ">="),
            Less => write!(fmt, "<"),
            Greater => write!(fmt, ">"),
        }
    }
}
