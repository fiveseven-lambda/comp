use crate::{ast, ir};
use std::collections::HashMap;

pub struct Context {
    variables_name: HashMap<String, usize>,
    num_variables: usize,
}

impl Context {
    pub fn new() -> Context {
        Context {
            variables_name: HashMap::new(),
            num_variables: 0,
        }
    }
    pub fn num_variables(&self) -> usize {
        self.num_variables
    }
    pub fn translate_expr(&mut self, expr: ast::Expr) -> ir::Expr {
        match expr {
            ast::Expr::Id(ty) => ir::Expr::Func {
                func: ir::Func::Id(translate_ty(ty)),
                calls: Vec::new(),
            },
            ast::Expr::Int(value) => ir::Expr::Int(value),
            ast::Expr::Call(func, args) => {
                let mut ret = self.translate_expr(*func);
                if let ir::Expr::Func { ref mut calls, .. } = ret {
                    calls.push(ir::Call::new(
                        args.into_iter()
                            .map(|arg| self.translate_expr(arg))
                            .collect(),
                    ));
                } else {
                    panic!("not a function");
                }
                ret
            }
            ast::Expr::Bin(left, op, right) => ir::Expr::Func {
                func: match op {
                    ast::BinOp::Add => ir::Func::Add,
                    ast::BinOp::Sub => ir::Func::Sub,
                    ast::BinOp::Mul => ir::Func::Mul,
                    ast::BinOp::Div => ir::Func::Div,
                    ast::BinOp::Rem => ir::Func::Rem,
                },
                calls: vec![ir::Call::new(vec![
                    self.translate_expr(*left),
                    self.translate_expr(*right),
                ])],
            },
            _ => ir::Expr::Func {
                func: ir::Func::Deref(ir::Ty::new(ir::TyInner::Undetermined)),
                calls: vec![ir::Call::new(vec![self.translate_ref(expr)])],
            },
        }
    }
    fn translate_ref(&mut self, expr: ast::Expr) -> ir::Expr {
        match expr {
            ast::Expr::Var(name) => {
                let var_idx = *self.variables_name.entry(name).or_insert_with(|| {
                    let new_idx = self.num_variables;
                    self.num_variables += 1;
                    new_idx
                });
                ir::Expr::Var(var_idx)
            }
            ast::Expr::Assign(left, right) => {
                let left = self.translate_ref(*left);
                let right = self.translate_expr(*right);
                ir::Expr::Func {
                    func: ir::Func::Assign(ir::Ty::new(ir::TyInner::Undetermined)),
                    calls: vec![ir::Call::new(vec![left, right])],
                }
            }
            _ => panic!("not a lvalue"),
        }
    }
}

fn translate_ty(ty: ast::Ty) -> ir::Ty {
    match ty {
        ast::Ty::Int => ir::Ty::new(ir::TyInner::Int),
        ast::Ty::Func { args, ret } => ir::Ty::new(ir::TyInner::Func {
            args: args.into_iter().map(translate_ty).collect(),
            ret: translate_ty(*ret),
        }),
    }
}
