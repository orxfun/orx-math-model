use crate::{elements::Elements, Model};
use crate::{MetaCatSetInt, MetaCatSetStr, MetaInt};
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};

#[test]
fn int_p1_cat_int() {
    // data
    let values = vec![1usize, 2, 7, 9];

    // modeling
    let m = Model::default();
    let set = m.cat_set_int();
    let par = m.par_d1_int(set);

    // binding
    set.bind(|| values.iter().copied()).unwrap();

    // running
    let mut elements: Elements<usize> = vec![].into();
    unsafe { set.el.get() }.fill(&mut elements).unwrap();
    assert_eq!(elements.as_slice(), &[1, 2, 7, 9]);
}

#[test]
fn int_p1_cat_str() {
    // data
    let values: Vec<_> = [1, 2, 7, 9].map(|x| x.to_string()).into_iter().collect();

    // modeling
    let m = Model::default();
    let set = m.cat_set_str();

    // binding
    set.bind(|| &values).unwrap();

    // running
    let mut elements: Elements<&String> = vec![].into();
    unsafe { set.el.get() }.fill(&mut elements).unwrap();
    assert_eq!(elements.as_slice(), &["1", "2", "7", "9"]);
}

#[test]
fn int_p1_ord() {
    // data
    let values = vec![1i64, 2, 7, 9];

    // modeling
    let m = Model::default();
    let set = m.ord_set();

    // binding
    set.bind(|| values.iter().copied()).unwrap();

    // running
    let mut elements: Elements<i64> = vec![].into();
    unsafe { set.el.get() }.fill(&mut elements).unwrap();
    assert_eq!(elements.as_slice(), &[1, 2, 7, 9]);
}
