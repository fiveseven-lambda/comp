mod ast;
mod context;
mod ir;
mod parser;

use std::{cell::RefCell, io::BufRead, rc::Rc};

fn main() {
    let source = std::io::stdin().lock();
    let mut context = context::Context::new();
    let mut exprs: Vec<_> = source
        .lines()
        .filter_map(|line| {
            let line = line.expect("failed to read from stdin");
            parser::parse(&line).map(|expr| context.translate_expr(expr))
        })
        .collect();
    let vars: Vec<_> = (0..context.num_variables())
        .map(|_| {
            (
                ir::Ty::new(ir::TyInner::Ref(ir::Ty::new(ir::TyInner::Undetermined))),
                ir::Value::Var(Rc::new(RefCell::new(None))),
            )
        })
        .collect();
    for expr in &mut exprs {
        let (ty, value) = expr.eval(&vars);
        println!("{expr}: {ty}\n  -> {value}");
    }
}
