use crate::indices::DIdx;
use crate::pars::{DParIdx, ParIdx};
use crate::values::{DVal, Int};
use crate::{binding::Binding, ptr::RefPtr, sets::DSet};
use crate::{index_types::IdxIn, index_types::IdxInVariant, pars::io1::Io1, values::ValVariant};
use crate::{values::Value, Model};
use alloc::boxed::Box;
use alloc::string::String;
use core::ops::Index;
use orx_imp_vec::Elem;

// data

pub enum FParD1<'d, V> {
    CatUsize(Box<dyn Fn(usize) -> V + 'd>),
    CatStr(Box<dyn Fn(&'d String) -> V + 'd>),
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
        assert!(self.is_bound());
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

// symbol

#[derive(derive_new::new, Clone, Copy)]
#[new(visibility = "pub(crate)")]
pub struct IntParD1<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, DParD1<'d>>,
}

#[derive(derive_new::new, Clone, Copy)]
#[new(visibility = "pub(crate)")]
pub struct NumParD1<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, DParD1<'d>>,
}

impl<'m, 'd> IntParD1<'m, 'd> {
    pub fn at<I>(self, i0: I) -> Int<'m, 'd>
    where
        I: Into<ParIdx<'m, 'd>>,
    {
        unsafe { self.at_unchecked(i0) }
    }

    unsafe fn at_unchecked<I>(self, i0: I) -> Int<'d, 'd>
    where
        I: Into<ParIdx<'m, 'd>>,
    {
        let par = unsafe { self.el.get() };
        let i0 = i0.into().into_data();

        let data = match (par, i0) {
            (DParD1::Int(_, _, FParD1::CatUsize(f)), DParIdx::Idx(DIdx::CatUsize(_, i0))) => {
                let f = unsafe { RefPtr::new(f) };
                let fun = Box::new(move || f.val()(i0.val()));
                DVal::Int(fun)
            }
            (DParD1::Int(_, _, FParD1::CatUsize(f)), DParIdx::RawUsize(i0)) => {
                let f = unsafe { RefPtr::new(f) };
                let fun = Box::new(move || f.val()(i0));
                DVal::Int(fun)
            }

            (DParD1::Int(_, _, FParD1::CatStr(f)), DParIdx::Idx(DIdx::CatStr(_, i0))) => {
                let f = unsafe { RefPtr::new(f) };
                let fun = Box::new(move || f.val()(i0.val()));
                DVal::Int(fun)
            }
            (DParD1::Int(_, _, FParD1::CatStr(f)), DParIdx::RawStr(i0)) => {
                let f = unsafe { RefPtr::new(f) };
                let fun = Box::new(move || f.val()(i0));
                DVal::Int(fun)
            }

            (DParD1::Int(_, _, FParD1::Ord(f)), DParIdx::Idx(DIdx::Ord(_, i0))) => {
                let f = unsafe { RefPtr::new(f) };
                let fun = Box::new(move || f.val()(i0.val()));
                DVal::Int(fun)
            }
            (DParD1::Int(_, _, FParD1::Ord(f)), DParIdx::RawI64(i0)) => {
                let f = unsafe { RefPtr::new(f) };
                let fun = Box::new(move || f.val()(i0));
                DVal::Int(fun)
            }
            (DParD1::Int(_, _, FParD1::Ord(f)), DParIdx::Val(DVal::Int(i0))) => {
                let f = unsafe { RefPtr::new(f) };
                let i0 = unsafe { RefPtr::new(i0) };
                let fun = Box::new(move || f.val()((i0.val())()));
                DVal::Int(fun)
            }
            _ => unreachable!(),
        };
        let el = unsafe { self.m.val_for_idx(data) };
        Int::new(unsafe { &*(self.m as *const Model<'_>) }, el)
    }
}

impl<'m, 'd, I: Into<ParIdx<'m, 'd>>> Index<I> for IntParD1<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn index(&self, i0: I) -> &Self::Output {
        let x = unsafe { self.at_unchecked(i0) };
        unsafe { self.m.sym_int(x) }
    }
}
