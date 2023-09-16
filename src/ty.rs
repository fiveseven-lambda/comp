#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Ty {
    Int,
    Func { args: Vec<Ty>, ret: Box<Ty> },
}

use std::fmt::{self, Display, Formatter};
impl Display for Ty {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Ty::Int => write!(f, "Int"),
            Ty::Func { args, ret } => write!(
                f,
                "({}):{ret}",
                args.into_iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        }
    }
}
