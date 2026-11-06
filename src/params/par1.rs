use crate::indices::{CatIdx, DIdx, Idx};
use crate::ptr::RefPtr;
use crate::sets::{DSet, Set};
use crate::values::Num;
use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// data

pub enum FunParD1<'d, T> {
    CatInt(Box<dyn Fn(usize) -> T + 'd>),
    CatStr(Box<dyn Fn(&'d str) -> T + 'd>),
    Ord(Box<dyn Fn(usize) -> T + 'd>),
}

impl<'d, T> FunParD1<'d, T> {
    pub fn empty<'m>(set: impl Into<Set<'m, 'd>>) -> Self
    where
        'd: 'm,
        T: Default,
    {
        match set.into() {
            Set::CatInt(_) => Self::CatInt(Box::new(|_| Default::default())),
            Set::CatStr(_) => Self::CatStr(Box::new(|_| Default::default())),
            Set::Ord(_) => Self::Ord(Box::new(|_| Default::default())),
        }
    }
}

pub struct DParD1<'d, T> {
    s0: RefPtr<DSet<'d>>,
    fun: FunParD1<'d, T>,
}

impl<'d, T> DParD1<'d, T> {
    pub fn empty<'m>(set: impl Into<Set<'m, 'd>>) -> Self
    where
        T: Default,
        'd: 'm,
    {
        let set = set.into();
        let fun = FunParD1::empty(set);
        let s0 = RefPtr::from(set.el());
        Self { s0, fun }
    }
}

// sym

#[derive(derive_new::new, Clone, Copy)]
#[new(visibility = "pub(crate)")]
pub struct ParD1<'m, 'd, T> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, DParD1<'d, T>>,
}

impl<'m, 'd> ParD1<'m, 'd, f64> {
    pub fn at(self, i0: impl Into<Idx<'m, 'd>>) -> Num<'m, 'd> {
        let i0 = i0.into();
        let par = unsafe { self.el.get() };

        match (i0, &par.fun) {
            (Idx::CatInt(i0), FunParD1::CatInt(f)) => todo!(),
            _ => todo!(),
        }
    }
}
