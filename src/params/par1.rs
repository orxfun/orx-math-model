use crate::indices::{CatIdx, DIdx};
use crate::ptr::RefPtr;
use crate::sets::{DSet, Set};
use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// data

pub enum FunParD1<'d> {
    CatInt2Num(Box<dyn Fn(usize) -> f64 + 'd>),
    CatStr2Num(Box<dyn Fn(&'d str) -> f64 + 'd>),
    Ord2Num(Box<dyn Fn(usize) -> f64 + 'd>),
    CatInt2Int(Box<dyn Fn(usize) -> i64 + 'd>),
    CatStr2Int(Box<dyn Fn(&'d str) -> i64 + 'd>),
    Ord2Int(Box<dyn Fn(usize) -> i64 + 'd>),
}

impl<'d> FunParD1<'d> {
    pub fn empty_num<'m>(set: impl Into<Set<'m, 'd>>) -> Self
    where
        'd: 'm,
    {
        match set.into() {
            Set::CatInt(_) => Self::CatInt2Num(Box::new(|_| Default::default())),
            Set::CatStr(_) => Self::CatStr2Num(Box::new(|_| Default::default())),
            Set::Ord(_) => Self::Ord2Num(Box::new(|_| Default::default())),
        }
    }

    pub fn empty_int<'m>(set: impl Into<Set<'m, 'd>>) -> Self
    where
        'd: 'm,
    {
        match set.into() {
            Set::CatInt(_) => Self::CatInt2Int(Box::new(|_| Default::default())),
            Set::CatStr(_) => Self::CatStr2Int(Box::new(|_| Default::default())),
            Set::Ord(_) => Self::Ord2Int(Box::new(|_| Default::default())),
        }
    }
}

pub struct DParD1<'d> {
    s0: RefPtr<DSet<'d>>,
    fun: FunParD1<'d>,
}

impl<'d> DParD1<'d> {
    pub fn empty_num<'m>(set: impl Into<Set<'m, 'd>>) -> Self
    where
        'd: 'm,
    {
        let set = set.into();
        let fun = FunParD1::empty_num(set);
        let s0 = RefPtr::from(set.el());
        Self { s0, fun }
    }

    pub fn empty_int<'m>(set: impl Into<Set<'m, 'd>>) -> Self
    where
        'd: 'm,
    {
        let set = set.into();
        let fun = FunParD1::empty_int(set);
        let s0 = RefPtr::from(set.el());
        Self { s0, fun }
    }
}

// sym

#[derive(derive_new::new, Clone, Copy)]
#[new(visibility = "pub(crate)")]
pub struct ParD1<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, DParD1<'d>>,
}

impl<'m, 'd> ParD1<'m, 'd> {
    pub fn at(self) {
        todo!()
    }
}
