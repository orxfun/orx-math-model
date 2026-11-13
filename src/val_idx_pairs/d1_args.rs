use crate::pars::p1::ParD1;
use crate::sets::IndexCatInt;
use crate::val_idx_pairs::D1ValIdxPair;
use crate::{Ptr, Set, SetCatInt, SetCatStr, SetOrd, Val};

pub trait D1Args: Set {
    fn par_at<'m, 'd, V: Val>(
        par: ParD1<'m, 'd, V, Self>,
        idx: impl Into<Self::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd>
    where
        (V, Self): D1ValIdxPair<V = V, I0 = Self>;
}

// cat int

impl D1Args for SetCatInt {
    fn par_at<'m, 'd, V: Val>(
        par: ParD1<'m, 'd, V, Self>,
        idx: impl Into<Self::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd>
    where
        (V, Self): D1ValIdxPair<V = V, I0 = Self>,
    {
        let fun = Ptr::new(par.el);
        match idx.into() {
            IndexCatInt::Idx(idx) => {
                let idx = Ptr::new(idx.el);
                par.m.val::<V>(move || {
                    let fun = fun.get();
                    let idx = idx.get().read::<Self>();
                    fun.at::<V, Self>(idx)
                })
            }
            IndexCatInt::ConUsize(con) => par.m.val::<V>(move || {
                let fun = fun.get();
                fun.at::<V, Self>(con)
            }),
        }
    }
}

// cat set

impl D1Args for SetCatStr {
    fn par_at<'m, 'd, V: Val>(
        par: ParD1<'m, 'd, V, Self>,
        idx: impl Into<Self::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd>
    where
        (V, Self): D1ValIdxPair<V = V, I0 = Self>,
    {
        todo!()
    }
}

// ord

impl D1Args for SetOrd {
    fn par_at<'m, 'd, V: Val>(
        par: ParD1<'m, 'd, V, Self>,
        idx: impl Into<Self::IdxAny<'m, 'd>>,
    ) -> V::Sym<'m, 'd>
    where
        (V, Self): D1ValIdxPair<V = V, I0 = Self>,
    {
        todo!()
    }
}
