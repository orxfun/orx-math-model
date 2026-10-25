use crate::symbols::{Indices, IndicesFun};
use crate::symbols::{Set, SetFun};

pub trait IndexingSymbol<'d> {
    type Fun;
}

// categorical

impl<'m, 'd> IndexingSymbol<'d> for Set<'m, 'd> {
    type Fun = SetFun<'d>;
}

// indices

impl<'m, 'd> IndexingSymbol<'d> for Indices<'m, 'd> {
    type Fun = IndicesFun<'d>;
}
