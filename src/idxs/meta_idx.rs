use crate::idxs::{CatIdxInt, CatIdxStr, IdxSlot, OrdIdx};
use crate::{MetaCatSetInt, MetaCatSetStr, MetaOrdSet, MetaSet};

// symbols

pub trait Idx<'m, 'd> {
    type Meta: MetaSet;

    fn slot(self) -> IdxSlot<'d>;
}

impl<'m, 'd> Idx<'m, 'd> for CatIdxInt<'m, 'd> {
    type Meta = MetaCatSetInt;

    fn slot(self) -> IdxSlot<'d> {
        unsafe { self.el.get() }.idx.clone()
    }
}

impl<'m, 'd> Idx<'m, 'd> for CatIdxStr<'m, 'd> {
    type Meta = MetaCatSetStr;

    fn slot(self) -> IdxSlot<'d> {
        unsafe { self.el.get() }.idx.clone()
    }
}

impl<'m, 'd> Idx<'m, 'd> for OrdIdx<'m, 'd> {
    type Meta = MetaOrdSet;

    fn slot(self) -> IdxSlot<'d> {
        unsafe { self.el.get() }.idx.clone()
    }
}
