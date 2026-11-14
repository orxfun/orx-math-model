use crate::sets::CatInt;
use crate::{Model, SetCatInt, SetCatStr, SetOrd, ValInt, ValNum};
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use std::collections::HashMap;

// int

#[test]
fn p2_int_cint_cstr() {
    type V = ValInt;
    type I0 = SetCatInt;
    type I1 = SetCatStr;

    // data
    let values = |i0: usize| {
        [1, 2, 7, 9]
            .into_iter()
            .enumerate()
            .map(|(i, v)| (i.to_string(), (10 * i0 + v) as i64))
            .collect::<HashMap<_, _>>()
    };
    let par_data: Vec<HashMap<String, i64>> = (0..4).map(|i0| values(i0)).collect();

    // modeling
    let m = Model::default();
    let s1 = m.cat_set_int();
    let s2 = m.cat_set_str();
    let par = m.par2_int(s1, s2);

    // binding
    par.bind(|i, j| par_data[i][j]).unwrap();

    // running
    for (i, map) in par_data.iter().enumerate() {
        for (j, exp) in map.iter() {
            let val = unsafe { par.el.get() }.at::<V, I0, I1>(i, j);
            assert_eq!(*exp, val);
        }
    }
}

#[test]
fn p2_int_cstr_ord() {
    type V = ValInt;
    type I0 = SetCatStr;
    type I1 = SetOrd;

    // data
    let values = vec![vec![0i64, 1], vec![2, 3, 4], vec![5]];
    let par_data: HashMap<String, Vec<i64>> = values
        .into_iter()
        .enumerate()
        .map(|(i, v)| (i.to_string(), v))
        .collect();

    // modeling
    let m = Model::default();
    let s1 = m.cat_set_str();
    let s2 = m.ord_set();
    let par = m.par2_int(s1, s2);

    // binding
    par.bind(|i, j| par_data[i][j as usize]).unwrap();

    // running
    for (i, vec) in par_data.iter() {
        for (j, exp) in vec.iter().enumerate() {
            let val = unsafe { par.el.get() }.at::<V, I0, I1>(i, j as i64);
            assert_eq!(*exp, val);
        }
    }
}

#[test]
fn p2_int_ord_cint() {
    type V = ValInt;
    type I0 = SetOrd;
    type I1 = SetCatInt;

    // data
    let par_data = vec![vec![0i64, 1], vec![2, 3, 4], vec![5]];

    // modeling
    let m = Model::default();
    let s1 = m.ord_set();
    let s2 = m.cat_set_int();
    let par = m.par2_int(s1, s2);

    // binding
    par.bind(|i, j| par_data[i as usize][j]).unwrap();

    // running
    for (i, vec) in par_data.iter().enumerate() {
        for (j, exp) in vec.iter().enumerate() {
            let val = unsafe { par.el.get() }.at::<V, I0, I1>(i as i64, j);
            assert_eq!(*exp, val);
        }
    }
}

// num

#[test]
fn p2_num_cint_cstr() {
    type V = ValNum;
    type I0 = SetCatInt;
    type I1 = SetCatStr;

    // data
    let values = |i0: usize| {
        [1, 2, 7, 9]
            .into_iter()
            .enumerate()
            .map(|(i, v)| (i.to_string(), (10 * i0 + v) as f64))
            .collect::<HashMap<_, _>>()
    };
    let par_data: Vec<HashMap<String, f64>> = (0..4).map(|i0| values(i0)).collect();

    // modeling
    let m = Model::default();
    let s1 = m.cat_set_int();
    let s2 = m.cat_set_str();
    let par = m.par2_num(s1, s2);

    // binding
    par.bind(|i, j| par_data[i][j]).unwrap();

    // running
    for (i, map) in par_data.iter().enumerate() {
        for (j, exp) in map.iter() {
            let val = unsafe { par.el.get() }.at::<V, I0, I1>(i, j);
            assert_eq!(*exp, val);
        }
    }
}

#[test]
fn p2_num_cstr_ord() {
    type V = ValNum;
    type I0 = SetCatStr;
    type I1 = SetOrd;

    // data
    let values = vec![vec![0.0, 1.0], vec![2.0, 3.0, 4.0], vec![5.0]];
    let par_data: HashMap<String, Vec<f64>> = values
        .into_iter()
        .enumerate()
        .map(|(i, v)| (i.to_string(), v))
        .collect();

    // modeling
    let m = Model::default();
    let s1 = m.cat_set_str();
    let s2 = m.ord_set();
    let par = m.par2_num(s1, s2);

    // binding
    par.bind(|i, j| par_data[i][j as usize]).unwrap();

    // running
    for (i, vec) in par_data.iter() {
        for (j, exp) in vec.iter().enumerate() {
            let val = unsafe { par.el.get() }.at::<V, I0, I1>(i, j as i64);
            assert_eq!(*exp, val);
        }
    }
}

#[test]
fn p2_num_ord_cint() {
    type V = ValNum;
    type I0 = SetOrd;
    type I1 = SetCatInt;

    // data
    let par_data = vec![vec![0.0, 1.0], vec![2.0, 3.0, 4.0], vec![5.0]];

    // modeling
    let m = Model::default();
    let s1 = m.ord_set();
    let s2 = m.cat_set_int();
    let par = m.par2_num(s1, s2);

    // binding
    par.bind(|i, j| par_data[i as usize][j]).unwrap();

    // running
    for (i, vec) in par_data.iter().enumerate() {
        for (j, exp) in vec.iter().enumerate() {
            let val = unsafe { par.el.get() }.at::<V, I0, I1>(i as i64, j);
            assert_eq!(*exp, val);
        }
    }
}
