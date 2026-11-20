use crate::{Model, SetCatInt, SetCatStr, SetOrd, SetSym, SymWrap};
use alloc::string::{String, ToString};

#[test]
fn idx_cat_int() {
    type S = SetCatInt;

    // modeling
    let m = Model::new();
    let set = m.cat_set_int("S");
    let idx = set.idx();

    // running
    assert_eq!(idx.data().read::<S>(), 0);

    idx.data().write::<S>(42);
    assert_eq!(idx.data().read::<S>(), 42);
}

#[test]
fn idx_cat_str() {
    type S = SetCatStr;

    // data
    let s = 42.to_string();

    // modeling
    let m = Model::new();
    let set = m.cat_set_str("S");
    let idx = set.idx();

    // running
    assert_eq!(idx.data().read::<S>(), &String::new());

    idx.data().write::<S>(&s);
    assert_eq!(idx.data().read::<S>(), &s);
}

#[test]
fn idx_ord() {
    type S = SetOrd;

    // modeling
    let m = Model::new();
    let set = m.ord_set("S");
    let idx = set.idx();

    // running
    assert_eq!(idx.data().read::<S>(), 0);

    idx.data().write::<S>(42);
    assert_eq!(idx.data().read::<S>(), 42);
}
