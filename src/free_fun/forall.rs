use crate::idxs::SumIdxs;

pub struct ForAll<'m, 'd>(pub(crate) SumIdxs<'m, 'd>);

pub fn forall<'m, 'd>(indices: impl Into<SumIdxs<'m, 'd>>) -> ForAll<'m, 'd> {
    ForAll(indices.into())
}
