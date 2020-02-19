use std::fmt::{Debug, Error, Formatter};

pub enum Expr {
    Unit,
    Number(i32),
    BinOp(Box<Expr>, Opcode, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    //    Lambda,
    //    Application,
    //    Let,
    Error,
}

#[derive(Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Number(n) => write!(fmt, "{:?}", n),
            BinOp(ref l, op, ref r) => write!(fmt, "({:?} {:?} {:?})", l, op, r),
            Error => write!(fmt, "error"),
            Unit => write!(fmt, "()"),
            If(ref pred, ref if_true, ref if_false) => {
                write!(fmt, "if {:?} then {:?} else {:?}", pred, if_true, if_false)
            }
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
        }
    }
}
