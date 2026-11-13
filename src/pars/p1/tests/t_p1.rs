use crate::sets::CatInt;
use crate::{Model, SetCatInt, SetCatStr, SetOrd, ValInt, ValNum};
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use std::collections::HashMap;

// int

#[test]
fn p1_int_cint() {
    type I0 = SetCatInt;
    type V = ValInt;

    // data
    let values = vec![1i64, 2, 7, 9];

    // modeling
    let m = Model::default();
    let set = m.cat_set_int();
    let par = m.par1_int(set);

    // binding
    par.bind(|i| values[i]).unwrap();

    // running
    for (i, v) in values.iter().copied().enumerate() {
        let val = unsafe { par.el.get() }.t_at::<V, I0>(i);
        assert_eq!(v, val);
    }
}

#[test]
fn p1_int_cstr() {
    type I0 = SetCatStr;
    type V = ValInt;

    // data
    let values: HashMap<_, _> = [1i64, 2, 7, 9]
        .into_iter()
        .enumerate()
        .map(|(i, v)| (i.to_string(), v))
        .collect();

    // modeling
    let m = Model::default();
    let set = m.cat_set_str();
    let par = m.par1_int(set);

    // binding
    par.bind(|i| values[i]).unwrap();

    // running
    for (i, v) in values.iter() {
        let val = unsafe { par.el.get() }.t_at::<V, I0>(i);
        assert_eq!(*v, val);
    }
}

#[test]
fn p1_int_ord() {
    type I0 = SetOrd;
    type V = ValInt;

    // data
    let values = |i: i64| i * 2;

    // modeling
    let m = Model::default();
    let set = m.ord_set();
    let par = m.par1_int(set);

    // binding
    par.bind(|i| values(i)).unwrap();

    // running
    for (i, v) in (0..4).map(|i| values(i)).enumerate() {
        let val = unsafe { par.el.get() }.t_at::<V, I0>(i as i64);
        assert_eq!(v, val);
    }
}

// num

#[test]
fn p1_num_cint() {
    type I0 = SetCatInt;
    type V = ValNum;

    // data
    let values = vec![1.0, 2.0, 7.0, 9.0];

    // modeling
    let m = Model::default();
    let set = m.cat_set_int();
    let par = m.par1_num(set);

    // binding
    par.bind(|i| values[i]).unwrap();

    // running
    for (i, v) in values.iter().copied().enumerate() {
        let val = unsafe { par.el.get() }.t_at::<V, I0>(i);
        assert_eq!(v, val);
    }
}

#[test]
fn p1_num_cstr() {
    type I0 = SetCatStr;
    type V = ValNum;

    // data
    let values: HashMap<_, _> = [1.0, 2.0, 7.0, 9.0]
        .into_iter()
        .enumerate()
        .map(|(i, v)| (i.to_string(), v))
        .collect();

    // modeling
    let m = Model::default();
    let set = m.cat_set_str();
    let par = m.par1_num(set);

    // binding
    par.bind(|i| values[i]).unwrap();

    // running
    for (i, v) in values.iter() {
        let val = unsafe { par.el.get() }.t_at::<V, I0>(i);
        assert_eq!(*v, val);
    }
}

#[test]
fn p1_num_ord() {
    type I0 = SetOrd;
    type V = ValNum;

    // data
    let values = |i: i64| i as f64 * 2.0;

    // modeling
    let m = Model::default();
    let set = m.ord_set();
    let par = m.par1_num(set);

    // binding
    par.bind(|i| values(i)).unwrap();

    // running
    for (i, v) in (0..4).map(|i| values(i)).enumerate() {
        let val = unsafe { par.el.get() }.t_at::<V, I0>(i as i64);
        assert_eq!(v, val);
    }
}
