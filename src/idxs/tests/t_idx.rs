use crate::idxs::IdxVal;
use crate::{elements::Elements, Model};
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};

#[test]
fn idx_cat_int() {
    // data
    let values = vec![1usize, 2, 7, 9];

    // modeling
    let m = Model::default();
    let set = m.cat_set_int();
    let idx = set.idx();

    // binding
    set.bind(|| values.iter().copied()).unwrap();

    // running
    let mut elements: Elements<usize> = vec![].into();
    let iter = idx.iter(&mut elements);
    for (i, _) in iter.enumerate() {
        assert_eq!(
            unsafe { idx.el.get() }.idx.test_value(),
            IdxVal::CatIdxInt(values[i])
        );
    }
}

#[test]
fn idx_cat_str() {
    // data
    let values: Vec<_> = [1, 2, 7, 9].map(|x| x.to_string()).into_iter().collect();

    // modeling
    let m = Model::default();
    let set = m.cat_set_str();
    let idx = set.idx();

    // binding
    set.bind(|| &values).unwrap();

    // running
    let mut elements: Elements<&String> = vec![].into();
    let iter = idx.iter(&mut elements);
    for (i, _) in iter.enumerate() {
        assert_eq!(
            unsafe { idx.el.get() }.idx.test_value(),
            IdxVal::CatIdxStr(&values[i])
        );
    }
}

#[test]
fn idx_ord() {
    // data
    let values = vec![1i64, 2, 7, 9];

    // modeling
    let m = Model::default();
    let set = m.ord_set();
    let idx = set.idx();

    // binding
    set.bind(|| values.iter().copied()).unwrap();

    // running
    let mut elements: Elements<i64> = vec![].into();
    let iter = idx.iter(&mut elements);
    for (i, _) in iter.enumerate() {
        assert_eq!(
            unsafe { idx.el.get() }.idx.test_value(),
            IdxVal::OrdIdx(values[i])
        );
    }
}
