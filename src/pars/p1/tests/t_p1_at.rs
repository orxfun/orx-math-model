use crate::sets::CatInt;
use crate::{Model, SetCatInt, SetCatStr, SetOrd, ValInt, ValNum};
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use std::collections::HashMap;

// int

#[test]
fn p1_int_cint_at() {
    type I0 = SetCatInt;
    type V = ValInt;

    // data
    let set_data = vec![0, 2, 3];
    let par_data = vec![1i64, 2, 7, 9];

    // modeling
    let m = Model::default();
    let set = m.cat_set_int();
    let idx = set.idx();
    let par = m.par1_int(set);
    let sca1 = par.at(idx);

    // binding
    par.bind(|i| par_data[i]).unwrap();

    // running
    for (i, v) in par_data.iter().copied().enumerate() {
        let val = unsafe { par.el.get() }.at::<V, I0>(i);
        assert_eq!(v, val);
    }
}

#[test]
fn p1_int_cstr_at() {
    type I0 = SetCatStr;
    type V = ValInt;

    // data
    let set_data: Vec<_> = [0, 2, 3].map(|x| x.to_string()).into_iter().collect();
    let par_data: HashMap<_, _> = [1i64, 2, 7, 9]
        .into_iter()
        .enumerate()
        .map(|(i, v)| (i.to_string(), v))
        .collect();

    // modeling
    let m = Model::default();
    let set = m.cat_set_str();
    let idx = set.idx();
    let par = m.par1_int(set);

    // binding
    par.bind(|i| par_data[i]).unwrap();

    // running
    for (i, v) in par_data.iter() {
        let val = unsafe { par.el.get() }.at::<V, I0>(i);
        assert_eq!(*v, val);
    }
}

#[test]
fn p1_int_ord_at() {
    type I0 = SetOrd;
    type V = ValInt;

    // data
    let set_data = vec![0i64, 2, 3];
    let par_data = |i: i64| i * 2;

    // modeling
    let m = Model::default();
    let set = m.ord_set();
    let idx = set.idx();
    let par = m.par1_int(set);

    // binding
    par.bind(|i| par_data(i)).unwrap();

    // running
    for (i, v) in (0..4).map(|i| par_data(i)).enumerate() {
        let val = unsafe { par.el.get() }.at::<V, I0>(i as i64);
        assert_eq!(v, val);
    }
}

// num

#[test]
fn p1_num_cint_at() {
    type I0 = SetCatInt;
    type V = ValNum;

    // data
    let set_data = vec![0, 2, 3];
    let par_data = vec![1.0, 2.0, 7.0, 9.0];

    // modeling
    let m = Model::default();
    let set = m.cat_set_int();
    let idx = set.idx();
    let par = m.par1_num(set);

    // binding
    par.bind(|i| par_data[i]).unwrap();

    // running
    for (i, v) in par_data.iter().copied().enumerate() {
        let val = unsafe { par.el.get() }.at::<V, I0>(i);
        assert_eq!(v, val);
    }
}

#[test]
fn p1_num_cstr_at() {
    type I0 = SetCatStr;
    type V = ValNum;

    // data
    let set_data: Vec<_> = [0, 2, 3].map(|x| x.to_string()).into_iter().collect();
    let par_data: HashMap<_, _> = [1.0, 2.0, 7.0, 9.0]
        .into_iter()
        .enumerate()
        .map(|(i, v)| (i.to_string(), v))
        .collect();

    // modeling
    let m = Model::default();
    let set = m.cat_set_str();
    let idx = set.idx();
    let par = m.par1_num(set);

    // binding
    par.bind(|i| par_data[i]).unwrap();

    // running
    for (i, v) in par_data.iter() {
        let val = unsafe { par.el.get() }.at::<V, I0>(i);
        assert_eq!(*v, val);
    }
}

#[test]
fn p1_num_ord_at() {
    type I0 = SetOrd;
    type V = ValNum;

    // data
    let set_data = vec![0i64, 2, 3];
    let par_data = |i: i64| i as f64 * 2.0;

    // modeling
    let m = Model::default();
    let set = m.ord_set();
    let idx = set.idx();
    let par = m.par1_num(set);

    // binding
    par.bind(|i| par_data(i)).unwrap();

    // running
    for (i, v) in (0..4).map(|i| par_data(i)).enumerate() {
        let val = unsafe { par.el.get() }.at::<V, I0>(i as i64);
        assert_eq!(v, val);
    }
}
