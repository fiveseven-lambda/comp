use super::*;
use std::fmt::{self, Display, Formatter};

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Int(value) => write!(f, "{value}"),
            Expr::Var(idx) => write!(f, "&v{idx}"),
            Expr::Func { func, calls } => {
                write!(f, "{func}")?;
                for call in calls {
                    write!(
                        f,
                        "({})",
                        call.args
                            .iter()
                            .map(|arg| format!("{arg}"))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )?;
                }
                Ok(())
            }
        }
    }
}

impl Display for Func {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Func::Id(ty) => write!(f, "Id[{ty}]"),
            Func::Add => write!(f, "Add"),
            Func::Sub => write!(f, "Sub"),
            Func::Mul => write!(f, "Mul"),
            Func::Div => write!(f, "Div"),
            Func::Rem => write!(f, "Rem"),
            Func::Assign(ty) => write!(f, "Assign[{ty}]"),
            Func::Deref(ty) => write!(f, "Deref[{ty}]"),
        }
    }
}

impl Display for Ty {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        /*{
            use std::hash::{DefaultHasher, Hash, Hasher};
            let mut hasher = DefaultHasher::new();
            Rc::as_ptr(&self.inner).hash(&mut hasher);
            write!(f, "{} ", hasher.finish() % 1000)?;
        }*/
        match *self.inner.borrow() {
            TyInner::Int => {
                write!(f, "Int")
            }
            TyInner::Ref(ref ty) => write!(f, "Ref[{ty}]"),
            TyInner::Undetermined => write!(f, "?"),
            TyInner::Func { ref args, ref ret } => {
                write!(
                    f,
                    "({}){ret}",
                    args.iter()
                        .map(|arg| format!("{arg}"))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            TyInner::SameAs(ref ty) => write!(f, "{ty}"),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(value) => write!(f, "{value}"),
            Value::Var(var) => match *var.borrow() {
                Some(ref value) => write!(f, "-> {}", value),
                None => write!(f, "uninitialized"),
            },
            Value::Id => write!(f, "Id"),
            Value::Add => write!(f, "Add"),
            Value::Sub => write!(f, "Sub"),
            Value::Mul => write!(f, "Mul"),
            Value::Div => write!(f, "Div"),
            Value::Rem => write!(f, "Rem"),
            Value::Assign => write!(f, "Assign"),
            Value::Deref => write!(f, "Deref"),
            Value::Curry => write!(f, "Curry"),
            Value::App(func, args) => write!(
                f,
                "{func}({})",
                args.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Value::Const(value) => write!(f, "const {value}"),
        }
    }
}
