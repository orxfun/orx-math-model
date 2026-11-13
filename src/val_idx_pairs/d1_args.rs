use crate::{pars::p1::ParD1, Set, Val};

pub trait D1Args: Set {
    fn par_at<'m, 'd, V: Val>(
        par: ParD1<'m, 'd, V, Self>,
        idx: impl Into<Self::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd>;
}
