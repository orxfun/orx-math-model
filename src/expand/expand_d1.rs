use crate::idxs::IdxVec;
use crate::{Elements, ElementsIter, Set, SetCatInt, SetCatStr, SetOrd, SetTyp};
use alloc::vec::Vec;

pub fn expand_d1<'d, T>(idxs: &IdxVec<'d>, value: &T, op: impl FnMut(&T)) {
    match idxs.set_typ(0) {
        SetTyp::CatInt => expand_d1_i0::<T, SetCatInt>(idxs, value, op),
        SetTyp::CatStr => expand_d1_i0::<T, SetCatStr>(idxs, value, op),
        SetTyp::Ord => expand_d1_i0::<T, SetOrd>(idxs, value, op),
    }
}

fn expand_d1_i0<'d, T, I0: Set>(idxs: &IdxVec<'d>, value: &T, mut op: impl FnMut(&T)) {
    assert_eq!(idxs.len(), 1);
    let mut elem0: Elements<<I0 as Set>::IdxRaw<'d>> = Vec::new().into();

    let i0 = idxs[0];

    let i0 = ElementsIter::<I0>::new(i0, &mut elem0);
    for _ in i0 {
        op(value);
    }
}
