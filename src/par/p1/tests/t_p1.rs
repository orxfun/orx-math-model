use crate::vals::ValVal;
use crate::{MetaCatSetInt, MetaCatSetStr, MetaInt, MetaOrdSet, Model};
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use std::collections::HashMap;

// int

#[test]
fn int_p1_cat_int() {
    // data
    let values = vec![1i64, 2, 7, 9];

    // modeling
    let m = Model::default();
    let set = m.cat_set_int();
    let par = m.par_d1_int(set);

    // binding
    par.bind(|i| values[i]);

    // running
    for (i, v) in values.iter().copied().enumerate() {
        let val = unsafe { par.el.get() }.test_at::<MetaCatSetInt>(i).unwrap();
        assert_eq!(ValVal::Int(v), val);
    }
}

#[test]
fn int_p1_cat_str() {
    // data
    let values: HashMap<_, _> = [1i64, 2, 7, 9]
        .into_iter()
        .enumerate()
        .map(|(i, v)| (i.to_string(), v))
        .collect();

    // modeling
    let m = Model::default();
    let set = m.cat_set_str();
    let par = m.par_d1_int(set);

    // binding
    par.bind(|i| values[i]);

    // running
    for (i, v) in values.iter() {
        let val = unsafe { par.el.get() }.test_at::<MetaCatSetStr>(i).unwrap();
        assert_eq!(ValVal::Int(*v), val);
    }
}

#[test]
fn int_p1_ord() {
    // data
    let values = |i: i64| i * 2;

    // modeling
    let m = Model::default();
    let set = m.ord_set();
    let par = m.par_d1_int(set);

    // binding
    par.bind(|i| values(i));

    // running
    for (i, v) in (0..4).map(|i| values(i)).enumerate() {
        let val = unsafe { par.el.get() }
            .test_at::<MetaOrdSet>(i as i64)
            .unwrap();
        assert_eq!(ValVal::Int(v), val);
    }
}

// num

#[test]
fn num_p1_cat_int() {
    // data
    let values = vec![1.0, 2.0, 7.0, 9.0];

    // modeling
    let m = Model::default();
    let set = m.cat_set_int();
    let par = m.par_d1_num(set);

    // binding
    par.bind(|i| values[i]);

    // running
    for (i, v) in values.iter().copied().enumerate() {
        let val = unsafe { par.el.get() }.test_at::<MetaCatSetInt>(i).unwrap();
        assert_eq!(ValVal::Num(v), val);
    }
}

#[test]
fn num_p1_cat_str() {
    // data
    let values: HashMap<_, _> = [1.0, 2.0, 7.0, 9.0]
        .into_iter()
        .enumerate()
        .map(|(i, v)| (i.to_string(), v))
        .collect();

    // modeling
    let m = Model::default();
    let set = m.cat_set_str();
    let par = m.par_d1_num(set);

    // binding
    par.bind(|i| values[i]);

    // running
    for (i, v) in values.iter() {
        let val = unsafe { par.el.get() }.test_at::<MetaCatSetStr>(i).unwrap();
        assert_eq!(ValVal::Num(*v), val);
    }
}

#[test]
fn num_p1_ord() {
    // data
    let values = |i: i64| i as f64 * 2.0;

    // modeling
    let m = Model::default();
    let set = m.ord_set();
    let par = m.par_d1_num(set);

    // binding
    par.bind(|i| values(i));

    // running
    for (i, v) in (0..4).map(|i| values(i)).enumerate() {
        let val = unsafe { par.el.get() }
            .test_at::<MetaOrdSet>(i as i64)
            .unwrap();
        assert_eq!(ValVal::Num(v), val);
    }
}
