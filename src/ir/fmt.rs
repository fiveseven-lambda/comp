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
                    for args in &call.extra {
                        write!(
                            f,
                            "{{{}}}",
                            args.iter()
                                .map(|arg| format!("{arg}"))
                                .collect::<Vec<_>>()
                                .join(", ")
                        )?;
                    }
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
