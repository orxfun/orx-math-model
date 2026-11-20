use crate::idxs::{IdxCatInt, IdxCatStr, IdxOrd};
use crate::sets::{CatInt, CatStr, Ord};
use crate::{Set, SetCatInt, SetCatStr, SetOrd, SymWrap};

pub trait SetSym<'m, 'd: 'm>: SymWrap<'m, 'd> {
    type Meta: Set<SetSym<'m, 'd> = Self>;

    fn idx(self) -> <Self::Meta as Set>::IdxSym<'m, 'd>;
}

impl<'m, 'd> SetSym<'m, 'd> for CatInt<'m, 'd> {
    type Meta = SetCatInt;

    fn idx(self) -> IdxCatInt<'m, 'd> {
        self.m().idx::<SetCatInt>(self)
    }
}

impl<'m, 'd> SetSym<'m, 'd> for CatStr<'m, 'd> {
    type Meta = SetCatStr;

    fn idx(self) -> IdxCatStr<'m, 'd> {
        self.m().idx::<SetCatStr>(self)
    }
}

impl<'m, 'd> SetSym<'m, 'd> for Ord<'m, 'd> {
    type Meta = SetOrd;

    fn idx(self) -> IdxOrd<'m, 'd> {
        self.m().idx::<SetOrd>(self)
    }
}
