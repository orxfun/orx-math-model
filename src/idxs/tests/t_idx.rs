use crate::idxs::IdxVal;
use crate::{elements::Elements, Model};
use crate::{CatInt, CatStr, Ord};
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};

#[test]
fn idx_cat_int() {
    type S = CatInt;

    // modeling
    let m = Model::default();
    let set = m.cat_set_int();
    let idx = set.idx();

    // running
    assert_eq!(unsafe { idx.el.get() }.read::<S>(), 0);

    unsafe { idx.el.get() }.write::<S>(42);
    assert_eq!(unsafe { idx.el.get() }.read::<S>(), 42);
}

#[test]
fn idx_cat_str() {
    type S = CatStr;

    // data
    let s = 42.to_string();

    // modeling
    let m = Model::default();
    let set = m.cat_set_str();
    let idx = set.idx();

    // running
    assert_eq!(unsafe { idx.el.get() }.read::<S>(), &String::new());

    unsafe { idx.el.get() }.write::<S>(&s);
    assert_eq!(unsafe { idx.el.get() }.read::<S>(), &s);
}

#[test]
fn idx_ord() {
    type S = Ord;

    // modeling
    let m = Model::default();
    let set = m.ord_set();
    let idx = set.idx();

    // running
    assert_eq!(unsafe { idx.el.get() }.read::<S>(), 0);

    unsafe { idx.el.get() }.write::<S>(42);
    assert_eq!(unsafe { idx.el.get() }.read::<S>(), 42);
}
