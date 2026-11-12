use crate::vals::ValVal;
use crate::{Elements, MetaCatSetInt, MetaCatSetStr, MetaInt, MetaOrdSet, Model};
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use std::collections::HashMap;

// int

#[test]
fn int_p1_cat_int() {
    // data
    let set_data = vec![0, 2, 3];
    let par_data = vec![1i64, 2, 7, 9];

    // modeling
    let m = Model::default();
    let set = m.cat_set_int();
    let par = m.par_d1_int(set);
    let idx = set.idx();
    let sca = par.at(idx);

    // binding
    par.bind(|i| par_data[i]);
    set.bind(|| set_data.iter().copied()).unwrap();

    // running
    let mut elements: Elements<usize> = vec![].into();
    let iter = idx.iter(&mut elements);
    for (i, _) in iter.enumerate() {
        let exp = ValVal::Int(par_data[set_data[i]]);
        let val = unsafe { sca.el.get() }.test_eval();
        assert_eq!(exp, val);
    }
}

#[test]
fn int_p1_cat_str() {
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
    let par = m.par_d1_int(set);
    let idx = set.idx();
    let sca = par.at(idx);

    // binding
    par.bind(|i| par_data[i]);
    set.bind(|| &set_data).unwrap();

    // running
    let mut elements: Elements<&String> = vec![].into();
    let iter = idx.iter(&mut elements);
    for (i, _) in iter.enumerate() {
        let exp = ValVal::Int(par_data[&set_data[i]]);
        let val = unsafe { sca.el.get() }.test_eval();
        assert_eq!(exp, val);
    }
}

#[test]
fn int_p1_ord() {
    // data
    let set_data = vec![0i64, 2, 3];
    let par_data = |i: i64| i * 2;

    // modeling
    let m = Model::default();
    let set = m.ord_set();
    let par = m.par_d1_int(set);
    let idx = set.idx();
    let sca = par.at(idx);

    // binding
    par.bind(|i| par_data(i));
    set.bind(|| set_data.iter().copied()).unwrap();

    // running
    let mut elements: Elements<i64> = vec![].into();
    let iter = idx.iter(&mut elements);
    for (i, _) in iter.enumerate() {
        let exp = ValVal::Int(par_data(set_data[i]));
        let val = unsafe { sca.el.get() }.test_eval();
        assert_eq!(exp, val);
    }
}

// num

// #[test]
// fn num_p1_cat_int() {
//     // data
//     let set_data = vec![0, 2, 3];
//     let par_data = vec![1.0, 2.0, 7.0, 9.0];

//     // modeling
//     let m = Model::default();
//     let set = m.cat_set_int();
//     let par = m.par_d1_num(set);
//     let idx = set.idx();
//     let sca = par.at(idx);

//     // binding
//     par.bind(|i| par_data[i]);
//     set.bind(|| set_data.iter().copied()).unwrap();

//     // running
//     let mut elements: Elements<usize> = vec![].into();
//     let iter = idx.iter(&mut elements);
//     for (i, _) in iter.enumerate() {
//         let exp = ValVal::Int(par_data[set_data[i]]);
//         let val = unsafe { sca.el.get() }.test_eval();
//         assert_eq!(exp, val);
//     }
// }

// #[test]
// fn num_p1_cat_str() {
//     // data
//     let set_data: Vec<_> = [0, 2, 3].map(|x| x.to_string()).into_iter().collect();
//     let par_data: HashMap<_, _> = [1i64, 2, 7, 9]
//         .into_iter()
//         .enumerate()
//         .map(|(i, v)| (i.to_string(), v))
//         .collect();

//     // modeling
//     let m = Model::default();
//     let set = m.cat_set_str();
//     let par = m.par_d1_int(set);
//     let idx = set.idx();
//     let sca = par.at(idx);

//     // binding
//     par.bind(|i| par_data[i]);
//     set.bind(|| &set_data).unwrap();

//     // running
//     let mut elements: Elements<&String> = vec![].into();
//     let iter = idx.iter(&mut elements);
//     for (i, _) in iter.enumerate() {
//         let exp = ValVal::Int(par_data[&set_data[i]]);
//         let val = unsafe { sca.el.get() }.test_eval();
//         assert_eq!(exp, val);
//     }
// }

// #[test]
// fn num_p1_ord() {
//     // data
//     let set_data = vec![0i64, 2, 3];
//     let par_data = |i: i64| i * 2;

//     // modeling
//     let m = Model::default();
//     let set = m.ord_set();
//     let par = m.par_d1_int(set);
//     let idx = set.idx();
//     let sca = par.at(idx);

//     // binding
//     par.bind(|i| par_data(i));
//     set.bind(|| set_data.iter().copied()).unwrap();

//     // running
//     let mut elements: Elements<i64> = vec![].into();
//     let iter = idx.iter(&mut elements);
//     for (i, _) in iter.enumerate() {
//         let exp = ValVal::Int(par_data(set_data[i]));
//         let val = unsafe { sca.el.get() }.test_eval();
//         assert_eq!(exp, val);
//     }
// }
