use crate::sets::cat::DCatSet;
use crate::sets::ord::DOrdSet;
use crate::sets::{CatSet, OrdSet};
use orx_imp_vec::Elem;

use crate::{elements::Elements, index_types::IdxIn};
use alloc::boxed::Box;

// data

pub enum DSet2<'d> {
    CatUsize(Box<dyn Fn(&mut Elements<usize>) + 'd>),
    CatStr(Box<dyn Fn(&mut Elements<&'d str>) + 'd>),
    Ord(Box<dyn Fn(&mut Elements<i64>) + 'd>),
}

impl<'d> DSet2<'d> {
    pub fn new_cat() -> Self {
        Self::CatUsize(Box::new(|_| {}))
    }

    pub fn new_ord() -> Self {
        Self::Ord(Box::new(|_| {}))
    }

    pub fn bind<I: IdxIn<'d>>(&mut self, fun: impl Fn(&mut Elements<I>) + 'd) {
        I::bind_set(self, fun);
    }
}

// data

pub enum DSet<'d> {
    CatInt(DCatSet<'d, usize>),
    CatStr(DCatSet<'d, &'d str>),
    Ord(DOrdSet<'d>),
}

impl<'d> DSet<'d> {
    pub fn empty_cat_int() -> Self {
        Self::CatInt(DCatSet::empty())
    }

    pub fn empty_cat_str() -> Self {
        Self::CatStr(DCatSet::empty())
    }

    pub fn empty_ord() -> Self {
        Self::Ord(DOrdSet::empty())
    }

    // TODO: add panics
    pub fn as_cat_int(&self) -> &DCatSet<'d, usize> {
        match self {
            Self::CatInt(x) => x,
            _ => unreachable!(),
        }
    }

    // TODO: add panics
    pub fn as_cat_str(&self) -> &DCatSet<'d, &'d str> {
        match self {
            Self::CatStr(x) => x,
            _ => unreachable!(),
        }
    }

    // TODO: add panics
    pub fn as_ord(&self) -> &DOrdSet<'d> {
        match self {
            Self::Ord(x) => x,
            _ => unreachable!(),
        }
    }

    // TODO: add panics
    pub fn as_cat_int_mut(&mut self) -> &mut DCatSet<'d, usize> {
        match self {
            Self::CatInt(x) => x,
            _ => unreachable!(),
        }
    }

    // TODO: add panics
    pub fn as_cat_str_mut(&mut self) -> &mut DCatSet<'d, &'d str> {
        match self {
            Self::CatStr(x) => x,
            _ => unreachable!(),
        }
    }

    // TODO: add panics
    pub fn as_ord_mut(&mut self) -> &mut DOrdSet<'d> {
        match self {
            Self::Ord(x) => x,
            _ => unreachable!(),
        }
    }
}

// sym

#[derive(Clone, Copy)]
pub enum Set<'m, 'd> {
    CatInt(CatSet<'m, 'd, usize>),
    CatStr(CatSet<'m, 'd, &'d str>),
    Ord(OrdSet<'m, 'd>),
}

impl<'m, 'd> From<CatSet<'m, 'd, usize>> for Set<'m, 'd> {
    fn from(value: CatSet<'m, 'd, usize>) -> Self {
        Self::CatInt(value)
    }
}

impl<'m, 'd> From<CatSet<'m, 'd, &'d str>> for Set<'m, 'd> {
    fn from(value: CatSet<'m, 'd, &'d str>) -> Self {
        Self::CatStr(value)
    }
}

impl<'m, 'd> From<OrdSet<'m, 'd>> for Set<'m, 'd> {
    fn from(value: OrdSet<'m, 'd>) -> Self {
        Self::Ord(value)
    }
}

impl<'m, 'd> Set<'m, 'd> {
    pub fn el(&self) -> Elem<'m, DSet<'d>> {
        match self {
            Self::CatInt(x) => x.el,
            Self::CatStr(x) => x.el,
            Self::Ord(x) => x.el,
        }
    }
}
