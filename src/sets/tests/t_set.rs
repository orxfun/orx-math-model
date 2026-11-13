use crate::{elements::Elements, Model};
use crate::{MetaCatInt, MetaCatStr, MetaOrd};
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};

#[test]
fn set_cat_int() {
    // data
    let values = vec![1usize, 2, 7, 9];

    // modeling
    let m = Model::default();
    let set = m.cat_set_int();

    // binding
    set.bind(|| values.iter().copied()).unwrap();

    // running
    let mut el: Elements<usize> = vec![].into();
    unsafe { set.el.get() }.fill::<MetaCatInt>(&mut el).unwrap();
    assert_eq!(el.as_slice(), &[1, 2, 7, 9]);
}

#[test]
fn set_cat_str() {
    // data
    let values: Vec<_> = [1, 2, 7, 9].map(|x| x.to_string()).into_iter().collect();

    // modeling
    let m = Model::default();
    let set = m.cat_set_str();

    // binding
    set.bind(|| &values).unwrap();

    // running
    let mut el: Elements<&String> = vec![].into();
    unsafe { set.el.get() }.fill::<MetaCatStr>(&mut el).unwrap();
    assert_eq!(el.as_slice(), &["1", "2", "7", "9"]);
}

#[test]
fn set_ord() {
    // data
    let values = vec![1i64, 2, 7, 9];

    // modeling
    let m = Model::default();
    let set = m.ord_set();

    // binding
    set.bind(|| values.iter().copied()).unwrap();

    // running
    let mut el: Elements<i64> = vec![].into();
    unsafe { set.el.get() }.fill::<MetaOrd>(&mut el).unwrap();
    assert_eq!(el.as_slice(), &[1, 2, 7, 9]);
}

#[test]
fn abc() {
    // data
    let values = vec![1usize, 2, 7, 9];

    // modeling
    let m = Model::default();
    let set = m.cat_set_int();

    // binding
    set.bind(|| values.iter().copied()).unwrap();

    // running
    let mut el: Elements<usize> = vec![].into();
    unsafe { set.el.get() }.fill::<MetaCatInt>(&mut el).unwrap();
    assert_eq!(el.as_slice(), &[1, 2, 7, 9]);
}
