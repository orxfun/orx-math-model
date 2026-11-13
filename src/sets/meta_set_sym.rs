use crate::sets::{CatInt, CatStr, DSet, Ord};
use crate::{Set, SetCatInt, SetCatStr, SetOrd, Symbol};
use core::ops::Deref;

pub trait SetSym<'m, 'd: 'm>: Copy + Deref<Target = Symbol<'m, 'd, DSet<'d>>> {
    type Meta: Set;
}

impl<'m, 'd> SetSym<'m, 'd> for CatInt<'m, 'd> {
    type Meta = SetCatInt;
}

impl<'m, 'd> SetSym<'m, 'd> for CatStr<'m, 'd> {
    type Meta = SetCatStr;
}

impl<'m, 'd> SetSym<'m, 'd> for Ord<'m, 'd> {
    type Meta = SetOrd;
}
