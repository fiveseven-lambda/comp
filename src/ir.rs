mod fmt;
use std::{cell::RefCell, collections::VecDeque, iter, rc::Rc};

pub enum Expr {
    Int(i32),
    Var(usize),
    Func { func: Func, calls: Vec<Call> },
}

pub enum Func {
    Id(Ty),
    Deref(Ty),
    Assign(Ty),
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

pub struct Call {
    pub args: Vec<Expr>,
}

#[derive(Clone)]
pub enum Value {
    Int(i32),
    Var(Rc<RefCell<Option<Value>>>),
    Id,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Assign,
    Deref,
    Curry,
    App(Box<Value>, Vec<Value>),
    Const(Box<Value>),
}

impl Value {
    fn call(&self, args: &[Value]) -> Value {
        match self {
            Value::Id => args[0].clone(),
            Value::Add => match (&args[0], &args[1]) {
                (&Value::Int(x), &Value::Int(y)) => Value::Int(x + y),
                _ => panic!(),
            },
            Value::Sub => match (&args[0], &args[1]) {
                (&Value::Int(x), &Value::Int(y)) => Value::Int(x - y),
                _ => panic!(),
            },
            Value::Mul => match (&args[0], &args[1]) {
                (&Value::Int(x), &Value::Int(y)) => Value::Int(x * y),
                _ => panic!(),
            },
            Value::Div => match (&args[0], &args[1]) {
                (&Value::Int(x), &Value::Int(y)) => Value::Int(x / y),
                _ => panic!(),
            },
            Value::Rem => match (&args[0], &args[1]) {
                (&Value::Int(x), &Value::Int(y)) => Value::Int(x % y),
                _ => panic!(),
            },
            Value::Assign => match args[0] {
                Value::Var(ref var) => {
                    *var.borrow_mut() = Some(args[1].clone());
                    Value::Var(var.clone())
                }
                _ => panic!(),
            },
            Value::Deref => match args[0] {
                Value::Var(ref var) => var.borrow().clone().expect(""),
                _ => panic!(),
            },
            Value::Curry => Value::App(Box::new(args[0].clone()), args[1..].to_vec()),
            Value::App(func, converters) => {
                let converted_args: Vec<_> = converters
                    .iter()
                    .map(|converter| converter.call(args))
                    .collect();
                func.call(&converted_args)
            }
            Value::Const(value) => *value.clone(),
            _ => panic!("not a function"),
        }
    }
}

macro_rules! ty {
    (Int) => {
        Ty::new(TyInner::Int)
    };
    (Ref $ty:expr) => {
        Ty::new(TyInner::Ref($ty))
    };
    (($($args:expr),*) $ret:expr) => {
        Ty::new(TyInner::Func { args: vec![$($args),*], ret: $ret } )
    }
}

impl Expr {
    pub fn eval(&self, vars: &[(Ty, Value)]) -> (Ty, Value) {
        match *self {
            Expr::Int(value) => (ty!(Int), Value::Int(value)),
            Expr::Var(idx) => vars[idx].clone(),
            Expr::Func {
                ref func,
                ref calls,
            } => {
                let (mut ty, mut value) = match func {
                    Func::Id(ty) => (ty!((ty.clone()) ty.clone()), Value::Id),
                    Func::Add => (ty!((ty!(Int), ty!(Int)) ty!(Int)), Value::Add),
                    Func::Sub => (ty!((ty!(Int), ty!(Int)) ty!(Int)), Value::Sub),
                    Func::Mul => (ty!((ty!(Int), ty!(Int)) ty!(Int)), Value::Mul),
                    Func::Div => (ty!((ty!(Int), ty!(Int)) ty!(Int)), Value::Div),
                    Func::Rem => (ty!((ty!(Int), ty!(Int)) ty!(Int)), Value::Rem),
                    Func::Assign(ty) => (
                        ty!((ty!(Ref ty.clone()), ty.clone()) ty!(Ref ty.clone())),
                        Value::Assign,
                    ),
                    Func::Deref(ty) => (ty!((ty!(Ref ty.clone())) ty.clone()), Value::Deref),
                };
                for call in calls {
                    let (args_ty, ret_ty) = ty.get_args_ret();
                    assert_eq!(call.args.len(), args_ty.len());
                    let call_args: Vec<_> = args_ty
                        .iter()
                        .zip(&call.args)
                        .map(|(arg_ty, call_arg)| {
                            let (call_arg_ty, call_arg_value) = call_arg.eval(vars);
                            let extra_calls = call_arg_ty.unify(arg_ty).expect("type error");
                            (call_arg_value, extra_calls)
                        })
                        .collect();
                    let max_extra_calls = match call_args
                        .iter()
                        .max_by_key(|(_, extra_calls)| extra_calls.len())
                    {
                        Some((_, extra_calls)) => extra_calls.clone(),
                        None => VecDeque::new(),
                    };
                    let mut args: Vec<Value> = call_args
                        .into_iter()
                        .map(|(arg, extra_calls)| {
                            (extra_calls.len()..max_extra_calls.len())
                                .fold(arg, |value, _| Value::Const(Box::new(value)))
                        })
                        .collect();
                    for _ in &max_extra_calls {
                        args = iter::once(value).chain(args).collect();
                        value = Value::Curry;
                    }
                    ty = max_extra_calls
                        .into_iter()
                        .fold(ret_ty, |ret, args| Ty::new(TyInner::Func { args, ret }));
                    value = value.call(&args);
                }
                (ty, value)
            }
        }
    }
}

#[derive(Clone)]
pub struct Ty {
    inner: Rc<RefCell<TyInner>>,
}
impl Ty {
    pub fn new(inner: TyInner) -> Ty {
        Ty {
            inner: Rc::new(RefCell::new(inner)),
        }
    }
}

pub enum TyInner {
    Int,
    Ref(Ty),
    Func { args: Vec<Ty>, ret: Ty },
    Undetermined,
    SameAs(Ty),
}

impl Ty {
    fn get_args_ret(&self) -> (Vec<Ty>, Ty) {
        match *self.inner.borrow() {
            TyInner::SameAs(ref ty) => ty.get_args_ret(),
            TyInner::Func { ref args, ref ret } => (args.clone(), ret.clone()),
            _ => panic!("not a function"),
        }
    }
    fn unify(&self, other: &Ty) -> Option<VecDeque<Vec<Ty>>> {
        let self_binding = self.inner.borrow();
        let other_binding = other.inner.borrow();
        match (&*self_binding, &*other_binding) {
            (_, TyInner::SameAs(other_equiv)) => {
                drop(self_binding);
                self.unify(other_equiv)
            }
            (TyInner::SameAs(self_equiv), _) => {
                drop(other_binding);
                self_equiv.unify(other)
            }
            (_, TyInner::Undetermined) => {
                drop(other_binding);
                *other.inner.borrow_mut() = TyInner::SameAs(self.clone());
                Some(VecDeque::new())
            }
            (TyInner::Undetermined, _) => {
                drop(self_binding);
                *self.inner.borrow_mut() = TyInner::SameAs(other.clone());
                Some(VecDeque::new())
            }
            (TyInner::Int, TyInner::Int) => Some(VecDeque::new()),
            (TyInner::Ref(self_0), TyInner::Ref(other_0)) => self_0.unify(other_0),
            (_, TyInner::Func { args: _, ret }) => {
                drop(self_binding);
                let mut tmp = self.unify(ret)?;
                tmp.pop_front();
                Some(tmp)
            }
            (TyInner::Func { args, ret }, _) => {
                drop(other_binding);
                let mut tmp = ret.unify(other)?;
                tmp.push_back(args.clone());
                Some(tmp)
            }
            _ => None,
        }
    }
}
