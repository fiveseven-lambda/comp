mod ast;
mod context;
mod ir;
mod parser;

use std::io::BufRead;

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
    let vars_ty: Vec<_> = (0..context.num_variables())
        .map(|_| ir::Ty::new(ir::TyInner::Undetermined))
        .collect();
    for expr in &mut exprs {
        expr.get_ty(&vars_ty);
    }
    for (idx, ty) in vars_ty.iter().enumerate() {
        println!("v{idx}: {ty}");
    }
    for expr in &exprs {
        println!("{expr}");
    }
}
