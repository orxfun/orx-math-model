use crate::idxs::SumIdxs;

pub struct SumOver<'m, 'd>(pub(crate) SumIdxs<'m, 'd>);

pub fn over<'m, 'd>(indices: impl Into<SumIdxs<'m, 'd>>) -> SumOver<'m, 'd> {
    SumOver(indices.into())
}
