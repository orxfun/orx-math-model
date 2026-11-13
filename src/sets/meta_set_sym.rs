use crate::sets::{CatInt, CatStr, Ord};
use crate::{Set, SetCatInt, SetCatStr, SetOrd};

pub trait SetSym<'m, 'd> {
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
