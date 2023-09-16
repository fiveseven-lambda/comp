use crate::ty::Ty;
use crate::value::{Func, Value};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub enum Stmt {
    Comment,
    Decl(String, Expr),
    Expr(Expr),
}

#[derive(Debug)]
pub enum Expr {
    Int(u32),
    Id(String),
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
        need_convert: Option<Vec<bool>>,
    },
}

impl Expr {
    pub fn ty(&mut self, vars: &HashMap<String, (Ty, Value)>) -> Ty {
        match self {
            Expr::Int(_) => Ty::Int,
            Expr::Id(name) => vars.get(name).expect("no such variable").0.clone(),
            Expr::Call {
                func,
                args,
                need_convert,
            } => {
                let (expected_args_ty, ret_ty) = match func.ty(vars) {
                    Ty::Func { args, ret } => (args, ret),
                    _ => panic!("not a function"),
                };
                let args_ty: Vec<_> = args.iter_mut().map(|arg| arg.ty(vars)).collect();
                assert_eq!(
                    args_ty.len(),
                    expected_args_ty.len(),
                    "wrong number of arguments"
                );
                if args_ty
                    .iter()
                    .zip(&expected_args_ty)
                    .all(|(arg_ty, expected_arg_ty)| arg_ty == expected_arg_ty)
                {
                    return *ret_ty;
                }

                let mut args_ty_set = HashSet::new();
                let mut need_convert_tmp = vec![false; args_ty.len()];
                for ((arg_ty, expected_arg_ty), need_convert_cur) in args_ty
                    .into_iter()
                    .zip(&expected_args_ty)
                    .zip(&mut need_convert_tmp)
                {
                    if &arg_ty == expected_arg_ty {
                        *need_convert_cur = true;
                    } else {
                        match arg_ty {
                            Ty::Func { args, ret } if *ret == *expected_arg_ty => {
                                args_ty_set.insert(args);
                            }
                            _ => panic!("type mismatch"),
                        }
                    }
                }
                if args_ty_set.len() == 1 {
                    *need_convert = Some(need_convert_tmp);
                    return Ty::Func {
                        ret: ret_ty,
                        args: args_ty_set.into_iter().next().unwrap(),
                    };
                } else {
                    panic!("type mismatch")
                }
            }
        }
    }
    pub fn eval(self, vars: &HashMap<String, (Ty, Value)>) -> Value {
        match self {
            Expr::Int(value) => Value::Int(value),
            Expr::Id(name) => vars.get(&name).unwrap().1.clone(),
            Expr::Call {
                func,
                args,
                need_convert: None,
            } => {
                let Value::Func(func) = func.eval(vars) else {
                    panic!()
                };
                func.call(args.into_iter().map(|arg| arg.eval(vars)).collect())
            }
            Expr::Call {
                func,
                args,
                need_convert: Some(need_convert),
            } => {
                let Value::Func(func) = func.eval(vars) else {
                    panic!()
                };
                Value::Func(Func::Comp {
                    func: Box::new(func),
                    args: args
                        .into_iter()
                        .zip(need_convert)
                        .map(|(arg, need_convert_cur)| {
                            if need_convert_cur {
                                Func::Const(Box::new(arg.eval(vars)))
                            } else {
                                let Value::Func(arg) = arg.eval(vars) else {
                                    panic!()
                                };
                                arg
                            }
                        })
                        .collect(),
                })
            }
        }
    }
}
