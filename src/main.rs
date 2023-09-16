use std::collections::HashMap;

mod ast;
mod parser;
mod ty;
mod value;

fn main() {
    let source = std::io::stdin();
    let mut vars = HashMap::new();
    vars.insert(
        String::from("id"),
        (
            ty::Ty::Func {
                args: vec![ty::Ty::Int],
                ret: Box::new(ty::Ty::Int),
            },
            value::Value::Func(value::Func::Id),
        ),
    );
    vars.insert(
        String::from("add"),
        (
            ty::Ty::Func {
                args: vec![ty::Ty::Int, ty::Ty::Int],
                ret: Box::new(ty::Ty::Int),
            },
            value::Value::Func(value::Func::Add),
        ),
    );
    vars.insert(
        String::from("mul"),
        (
            ty::Ty::Func {
                args: vec![ty::Ty::Int, ty::Ty::Int],
                ret: Box::new(ty::Ty::Int),
            },
            value::Value::Func(value::Func::Mul),
        ),
    );
    loop {
        let mut s = String::new();
        if source
            .read_line(&mut s)
            .expect("failed to read from source")
            == 0
        {
            break;
        }
        let (_, stmt) = match parser::parse_stmt(&s) {
            Ok(stmt) => stmt,
            Err(err) => {
                eprintln!("{err}");
                break;
            }
        };
        match stmt {
            ast::Stmt::Comment => {}
            ast::Stmt::Decl(name, mut expr) => {
                let ty = expr.ty(&vars);
                let value = expr.eval(&vars);
                vars.insert(name, (ty, value));
            }
            ast::Stmt::Expr(mut expr) => {
                let ty = expr.ty(&vars);
                let value = expr.eval(&vars);
                println!("{value} : {ty}");
            }
        }
    }
}
