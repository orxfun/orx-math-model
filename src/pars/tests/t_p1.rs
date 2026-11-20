use crate::{Model, SetCatInt, SetCatStr, SetOrd, SymWrap};
use alloc::string::ToString;
use alloc::vec;
use std::collections::HashMap;

#[test]
fn p1_cint() {
    type I0 = SetCatInt;

    // data
    let values = vec![1.0, 2.0, 7.0, 9.0];

    // modeling
    let m = Model::new();
    let set = m.cat_set_int("S");
    let par = m.par1_num("p", set);

    // binding
    par.bind(|i| values[i]).unwrap();

    // running
    for (i, v) in values.iter().copied().enumerate() {
        let val = par.data().fun().d1_at::<I0>(i);
        assert_eq!(v, val);
    }
}

#[test]
fn p1_cstr() {
    type I0 = SetCatStr;

    // data
    let values: HashMap<_, _> = [1, 2, 7, 9]
        .into_iter()
        .enumerate()
        .map(|(i, v)| (i.to_string(), v))
        .collect();

    // modeling
    let m = Model::new();
    let set = m.cat_set_str("S");
    let par = m.par1_int("p", set);

    // binding
    par.bind(|i| values[i]).unwrap();

    // running
    for (i, v) in values.iter() {
        let val = par.data().fun().d1_at_int::<I0>(i);
        assert_eq!(*v, val);
    }
}

#[test]
fn p1_ord() {
    type I0 = SetOrd;

    // data
    let values = |i: i64| i as f64 * 2.0;

    // modeling
    let m = Model::new();
    let set = m.ord_set("S");
    let par = m.par1_num("p", set);

    // binding
    par.bind(|i| values(i)).unwrap();

    // running
    for (i, v) in (0..4).map(|i| values(i)).enumerate() {
        let val = par.data().fun().d1_at::<I0>(i as i64);
        assert_eq!(v, val);
    }
}
