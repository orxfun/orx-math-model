use crate::idxs::{CatIdxInt, CatIdxStr, IdxSlot, OrdIdx};
use crate::vals::Int;
use crate::{MetaCatSetInt, MetaCatSetStr, MetaOrdSet, MetaSet};
use alloc::string::String;

// symbols

pub trait Idx<'m, 'd> {
    type Meta: MetaSet;

    fn slot(self) -> IdxSlot<'d>;
}

// cat int

#[derive(derive_more::From)]
pub enum CatIntIdxSum<'m, 'd> {
    Idx(CatIdxInt<'m, 'd>),
    Raw(usize),
}

impl<'m, 'd> Idx<'m, 'd> for CatIdxInt<'m, 'd> {
    type Meta = MetaCatSetInt;

    fn slot(self) -> IdxSlot<'d> {
        unsafe { self.el.get() }.idx.clone()
    }
}

// cat str

#[derive(derive_more::From)]
pub enum CatStrIdxSum<'m, 'd> {
    Idx(CatIdxStr<'m, 'd>),
    RefString(&'d String),
    String(String),
    Str(&'d str),
}

impl<'m, 'd> Idx<'m, 'd> for CatIdxStr<'m, 'd> {
    type Meta = MetaCatSetStr;

    fn slot(self) -> IdxSlot<'d> {
        unsafe { self.el.get() }.idx.clone()
    }
}

// ord

#[derive(derive_more::From)]
pub enum OrdIdxSum<'m, 'd> {
    Idx(OrdIdx<'m, 'd>),
    Int(Int<'m, 'd>),
    Raw(i64),
}

impl<'m, 'd> Idx<'m, 'd> for OrdIdx<'m, 'd> {
    type Meta = MetaOrdSet;

    fn slot(self) -> IdxSlot<'d> {
        unsafe { self.el.get() }.idx.clone()
    }
}
