use crate::index_types::IdxInVariant;
use crate::values::ValVariant;
use crate::{binding::Binding, index_types::IdxIn, ptr::RefPtr, sets::DSet};
use crate::{pars::io1::Io1, values::Value};
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// data

pub enum FParD1<'d, V> {
    CatUsize(Box<dyn Fn(usize) -> V + 'd>),
    CatStr(Box<dyn Fn(&'d str) -> V + 'd>),
    Ord(Box<dyn Fn(i64) -> V + 'd>),
}

pub enum DParD1<'d> {
    Int(Binding, RefPtr<DSet<'d>>, FParD1<'d, i64>),
    Num(Binding, RefPtr<DSet<'d>>, FParD1<'d, f64>),
}

impl<'d> DParD1<'d> {
    pub fn new_int(set: Elem<'_, DSet<'d>>) -> Self {
        let set_ptr = RefPtr::from(set);
        match unsafe { set.get() } {
            DSet::CatUsize(_, _) => {
                let fun = FParD1::CatUsize(Box::new(|_| 0));
                Self::Int(Binding::Empty, set_ptr, fun)
            }
            DSet::CatStr(_, _) => {
                let fun = FParD1::CatStr(Box::new(|_| 0));
                Self::Int(Binding::Empty, set_ptr, fun)
            }
            DSet::Ord(_, _) => {
                let fun = FParD1::Ord(Box::new(|_| 0));
                Self::Int(Binding::Empty, set_ptr, fun)
            }
        }
    }

    pub fn new_num(set: Elem<'_, DSet<'d>>) -> Self {
        let set_ptr = RefPtr::from(set);
        match unsafe { set.get() } {
            DSet::CatUsize(_, _) => {
                let fun = FParD1::CatUsize(Box::new(|_| 0.0));
                Self::Num(Binding::Empty, set_ptr, fun)
            }
            DSet::CatStr(_, _) => {
                let fun = FParD1::CatStr(Box::new(|_| 0.0));
                Self::Num(Binding::Empty, set_ptr, fun)
            }
            DSet::Ord(_, _) => {
                let fun = FParD1::Ord(Box::new(|_| 0.0));
                Self::Num(Binding::Empty, set_ptr, fun)
            }
        }
    }

    pub fn is_bound(&self) -> bool {
        *(match self {
            Self::Int(b, _, _) => b,
            Self::Num(b, _, _) => b,
        }) == Binding::Bound
    }

    pub fn bind<I: IdxIn<'d>, V: Value>(&mut self, fun: impl Fn(I) -> V + 'd)
    where
        (I, V): Io1<'d, I0 = I, O = V>,
    {
        <(I, V) as Io1<'d>>::p1_bind(self, fun);
    }

    pub fn value_as_variant<I: IdxIn<'d>>(&self, idx: I) -> ValVariant {
        let idx: IdxInVariant = idx.into();
        match self {
            Self::Int(_, _, f) => match (idx, f) {
                (IdxInVariant::Usize(i0), FParD1::CatUsize(f)) => ValVariant::Int(f(i0)),
                (IdxInVariant::Str(i0), FParD1::CatStr(f)) => ValVariant::Int(f(i0)),
                (IdxInVariant::I64(i0), FParD1::Ord(f)) => ValVariant::Int(f(i0)),
                _ => unreachable!(),
            },
            Self::Num(_, _, f) => match (idx, f) {
                (IdxInVariant::Usize(i0), FParD1::CatUsize(f)) => ValVariant::Num(f(i0)),
                (IdxInVariant::Str(i0), FParD1::CatStr(f)) => ValVariant::Num(f(i0)),
                (IdxInVariant::I64(i0), FParD1::Ord(f)) => ValVariant::Num(f(i0)),
                _ => unreachable!(),
            },
        }
    }
}
