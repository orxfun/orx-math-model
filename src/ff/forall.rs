use crate::idxs::IdxSum;

pub struct ForAll<'m, 'd>(IdxSum<'m, 'd>);

pub fn forall<'m, 'd>(indices: impl Into<IdxSum<'m, 'd>>) -> ForAll<'m, 'd> {
    ForAll(indices.into())
}
