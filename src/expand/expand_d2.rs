use crate::idxs::IdxVec;
use crate::{Elements, ElementsIter, Set, SetCatInt, SetCatStr, SetOrd, SetTyp};
use alloc::vec::Vec;

pub fn expand_d2<'d, T>(idxs: &IdxVec<'d>, value: &T, op: impl FnMut(&T)) {
    match idxs.set_typ(0) {
        SetTyp::CatInt => expand_d2_i0::<T, SetCatInt>(idxs, value, op),
        SetTyp::CatStr => expand_d2_i0::<T, SetCatStr>(idxs, value, op),
        SetTyp::Ord => expand_d2_i0::<T, SetOrd>(idxs, value, op),
    }
}

fn expand_d2_i0<'d, T, I0: Set>(idxs: &IdxVec<'d>, value: &T, op: impl FnMut(&T)) {
    match idxs.set_typ(1) {
        SetTyp::CatInt => expand_d2_i0_i1::<T, I0, SetCatInt>(idxs, value, op),
        SetTyp::CatStr => expand_d2_i0_i1::<T, I0, SetCatStr>(idxs, value, op),
        SetTyp::Ord => expand_d2_i0_i1::<T, I0, SetOrd>(idxs, value, op),
    }
}

fn expand_d2_i0_i1<'d, T, I0: Set, I1: Set>(idxs: &IdxVec<'d>, value: &T, mut op: impl FnMut(&T)) {
    assert_eq!(idxs.len(), 2);
    let mut elem0: Elements<<I0 as Set>::IdxRaw<'d>> = Vec::new().into();
    let mut elem1: Elements<<I1 as Set>::IdxRaw<'d>> = Vec::new().into();

    let [i0, i1] = [0, 1].map(|i| idxs[i]);

    let i0 = ElementsIter::<I0>::new(i0, &mut elem0);
    for _ in i0 {
        let i1 = ElementsIter::<I1>::new(i1, &mut elem1);
        for _ in i1 {
            op(value);
        }
    }
}
