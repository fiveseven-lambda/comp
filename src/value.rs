#[derive(Clone)]
pub enum Value {
    Int(u32),
    Func(Func),
}

#[derive(Clone)]
pub enum Func {
    Id,
    Add,
    Mul,
    Const(Box<Value>),
    Comp { func: Box<Func>, args: Vec<Func> },
}

impl Func {
    pub fn call(self, args: Vec<Value>) -> Value {
        match self {
            Func::Add => {
                let mut args_iter = args.into_iter();
                match (args_iter.next().unwrap(), args_iter.next().unwrap()) {
                    (Value::Int(x), Value::Int(y)) => Value::Int(x + y),
                    _ => panic!(),
                }
            }
            Func::Mul => {
                let mut args_iter = args.into_iter();
                match (args_iter.next().unwrap(), args_iter.next().unwrap()) {
                    (Value::Int(x), Value::Int(y)) => Value::Int(x * y),
                    _ => panic!(),
                }
            }
            Func::Id => {
                let mut args_iter = args.into_iter();
                args_iter.next().unwrap()
            }
            Func::Const(value) => *value,
            Func::Comp { func, args: mids } => {
                func.call(mids.into_iter().map(|mid| mid.call(args.clone())).collect())
            }
        }
    }
}

use std::fmt::{self, Display, Formatter};
impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(value) => write!(f, "{value}"),
            Value::Func(func) => write!(f, "{func}"),
        }
    }
}
impl Display for Func {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Func::Add => write!(f, "add"),
            Func::Mul => write!(f, "mul"),
            Func::Id => write!(f, "id"),
            Func::Const(value) => write!(f, "const({value})"),
            Func::Comp { func, args } => {
                write!(
                    f,
                    "{func}.({})",
                    args.iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(",")
                )
            }
        }
    }
}
