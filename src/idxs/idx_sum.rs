use crate::idxs::{IdxCatInt, IdxCatStr, IdxOrd};

pub enum Idx<'m, 'd> {
    CatInt(IdxCatInt<'m, 'd>),
    CatStr(IdxCatStr<'m, 'd>),
    Ord(IdxOrd<'m, 'd>),
}

pub enum IdxSum<'m, 'd> {
    D1(Idx<'m, 'd>),
    D2(Idx<'m, 'd>, Idx<'m, 'd>),
    D3(Idx<'m, 'd>, Idx<'m, 'd>, Idx<'m, 'd>),
    D4(Idx<'m, 'd>, Idx<'m, 'd>, Idx<'m, 'd>, Idx<'m, 'd>),
}
