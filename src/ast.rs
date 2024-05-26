pub enum Expr {
    Id(Ty),
    Int(i32),
    Var(String),
    Assign(Box<Expr>, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    Bin(Box<Expr>, BinOp, Box<Expr>),
}

pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

pub enum Ty {
    Int,
    Func { args: Vec<Ty>, ret: Box<Ty> },
}
