use crate::sets::cat::DCatSet;
use crate::sets::ord::DOrdSet;
use crate::sets::{CatSet, OrdSet};

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
    pub fn into_cat_int(&self) -> &DCatSet<'d, usize> {
        match self {
            Self::CatInt(x) => x,
            _ => unreachable!(),
        }
    }

    // TODO: add panics
    pub fn into_cat_str(&self) -> &DCatSet<'d, &'d str> {
        match self {
            Self::CatStr(x) => x,
            _ => unreachable!(),
        }
    }

    // TODO: add panics
    pub fn into_ord(&self) -> &DOrdSet<'d> {
        match self {
            Self::Ord(x) => x,
            _ => unreachable!(),
        }
    }
}

// sym

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
