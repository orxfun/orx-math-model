use crate::symbols::{Element, Set, SetFun};
use crate::symbols::{Idx, Indices, IndicesFun};

pub trait IndexingSymbol<'d> {
    type Fun;

    type Idx;
}

// categorical

impl<'m, 'd> IndexingSymbol<'d> for Set<'m, 'd> {
    type Fun = SetFun<'d>;

    type Idx = Element<'m, 'd>;
}

// indices

impl<'m, 'd> IndexingSymbol<'d> for Indices<'m, 'd> {
    type Fun = IndicesFun<'d>;

    type Idx = Idx<'m, 'd>;
}
