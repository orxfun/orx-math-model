use crate::idxs::IdxVal;
use crate::{elements::Elements, Model};
use crate::{ElementsIter, MetaCatInt, MetaCatStr, MetaOrd, Set};
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};

#[test]
fn idx_set_iter_cat_int() {
    type S = MetaCatInt;
    type El<'d> = <S as Set>::IdxRaw<'d>;

    // data
    let values: Vec<El<'_>> = vec![1, 2, 7, 9];

    // modeling
    let m = Model::default();
    let set = m.cat_set_int();
    let idx = set.idx();

    // binding
    set.bind(|| values.iter().copied()).unwrap();

    // running
    let mut elements: Elements<El<'_>> = vec![].into();
    let iter = ElementsIter::<S>::new(idx, &mut elements);
    for (i, _) in iter.enumerate() {
        assert_eq!(unsafe { idx.el.get() }.read::<S>(), values[i]);
    }
}

#[test]
fn idx_set_iter_cat_str() {
    type S = MetaCatStr;
    type El<'d> = <S as Set>::IdxRaw<'d>;

    // data
    let values: Vec<_> = [1, 2, 7, 9].map(|x| x.to_string()).into_iter().collect();

    // modeling
    let m = Model::default();
    let set = m.cat_set_str();
    let idx = set.idx();

    // binding
    set.bind(|| &values).unwrap();

    // running
    let mut elements: Elements<El<'_>> = vec![].into();
    let iter = ElementsIter::<S>::new(idx, &mut elements);
    for (i, _) in iter.enumerate() {
        assert_eq!(unsafe { idx.el.get() }.read::<S>(), &values[i]);
    }
}

#[test]
fn idx_set_iter_ord() {
    type S = MetaOrd;
    type El<'d> = <S as Set>::IdxRaw<'d>;

    // data
    let values: Vec<El<'_>> = vec![1, 2, 7, 9];

    // modeling
    let m = Model::default();
    let set = m.ord_set();
    let idx = set.idx();

    // binding
    set.bind(|| values.iter().copied()).unwrap();

    // running
    let mut elements: Elements<El<'_>> = vec![].into();
    let iter = ElementsIter::<S>::new(idx, &mut elements);
    for (i, _) in iter.enumerate() {
        assert_eq!(unsafe { idx.el.get() }.read::<S>(), values[i]);
    }
}
