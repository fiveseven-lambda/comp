mod fmt;
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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

impl Expr {
    pub fn get_ty(&mut self, vars_ty: &[Ty]) -> Ty {
        match *self {
            Expr::Int(_) => Ty::new(TyInner::Int),
            Expr::Var(idx) => Ty::new(TyInner::Ref(vars_ty[idx].clone())),
            Expr::Func {
                ref func,
                ref mut calls,
            } => {
                let mut ty = match func {
                    Func::Id(ty) => Ty::new(TyInner::Func {
                        args: vec![ty.clone()],
                        ret: ty.clone(),
                    }),
                    Func::Add | Func::Sub | Func::Mul | Func::Div | Func::Rem => {
                        Ty::new(TyInner::Func {
                            args: vec![Ty::new(TyInner::Int), Ty::new(TyInner::Int)],
                            ret: Ty::new(TyInner::Int),
                        })
                    }
                    Func::Assign(ty) => Ty::new(TyInner::Func {
                        args: vec![Ty::new(TyInner::Ref(ty.clone())), ty.clone()],
                        ret: Ty::new(TyInner::Ref(ty.clone())),
                    }),
                    Func::Deref(ty) => Ty::new(TyInner::Func {
                        args: vec![Ty::new(TyInner::Ref(ty.clone()))],
                        ret: ty.clone(),
                    }),
                };
                for call in calls {
                    let (args, ret) = ty.get_args_ret();
                    assert_eq!(call.args.len(), args.len());
                    let mut max_extra_calls = VecDeque::new();
                    for (expected_arg, call_arg) in args.iter().zip(&mut call.args) {
                        let extra_calls = call_arg
                            .get_ty(vars_ty)
                            .unify(expected_arg)
                            .expect("type error");
                        if max_extra_calls.len() < extra_calls.len() {
                            max_extra_calls = extra_calls;
                        }
                    }
                    call.extra = max_extra_calls.clone();
                    ty = ret;
                    for args in max_extra_calls {
                        ty = Ty::new(TyInner::Func { args, ret: ty });
                    }
                }
                ty
            }
        }
    }
}

pub struct Call {
    pub args: Vec<Expr>,
    pub extra: VecDeque<Vec<Ty>>,
}
impl Call {
    pub fn new(args: Vec<Expr>) -> Call {
        Call {
            args,
            extra: VecDeque::new(),
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
