use crate::sets::CatInt;
use crate::{Elements, ElementsIter, Model, Set, SetCatInt, SetCatStr, SetOrd, ValInt, ValNum};
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use std::collections::HashMap;

// int

#[test]
fn p1_int_cint_at() {
    type I0 = SetCatInt;
    type V = ValInt;
    type El<'d> = <I0 as Set>::IdxRaw<'d>;

    // data
    let set_data = vec![0, 2, 3];
    let par_data = vec![1i64, 2, 7, 9];

    // modeling
    let m = Model::default();
    let set = m.cat_set_int();
    let idx = set.idx();
    let par = m.par1_int(set);
    let sca1 = [par.at(idx), par[idx]];
    let sca2 = [par.at(2), par[2]];

    // binding
    par.bind(|i| par_data[i]).unwrap();
    set.bind(|| set_data.iter().copied()).unwrap();

    // running
    let mut elements: Elements<El<'_>> = vec![].into();
    let iter = ElementsIter::<I0>::new(idx, &mut elements);
    for (i, _) in iter.enumerate() {
        let exp = par_data[set_data[i]];
        for sca in sca1 {
            assert_eq!(exp, unsafe { sca.el.get() }.eval::<V>());
        }
        let exp = par_data[set_data[2]];
        for sca in sca2 {
            assert_eq!(exp, unsafe { sca.el.get() }.eval::<V>());
        }
    }
}

#[test]
fn p1_int_cstr_at() {
    type I0 = SetCatStr;
    type V = ValInt;
    type El<'d> = <I0 as Set>::IdxRaw<'d>;

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
    let sca1 = [par.at(idx), par[idx]];
    let sca2 = [par.at(3.to_string()), par["3"], par[&set_data[2]]];

    // binding
    par.bind(|i| par_data[i]).unwrap();
    set.bind(|| &set_data).unwrap();

    // running
    let mut elements: Elements<El<'_>> = vec![].into();
    let iter = ElementsIter::<I0>::new(idx, &mut elements);
    for (i, _) in iter.enumerate() {
        let exp = par_data[&set_data[i]];
        for sca in sca1 {
            assert_eq!(exp, unsafe { sca.el.get() }.eval::<V>());
        }
        let exp = par_data[&set_data[2]];
        for sca in sca2 {
            assert_eq!(exp, unsafe { sca.el.get() }.eval::<V>());
        }
    }
}

#[test]
fn p1_int_ord_at() {
    type I0 = SetOrd;
    type V = ValInt;
    type El<'d> = <I0 as Set>::IdxRaw<'d>;

    // data
    let set_data = vec![0i64, 2, 3];
    let par_data = |i: i64| i * 2;

    // modeling
    let m = Model::default();
    let set = m.ord_set();
    let idx = set.idx();
    let par = m.par1_int(set);
    let sca1 = [par.at(idx), par[idx]];
    let sca2 = [par.at(2), par[2]];
    // TODO: add sca3 to test Int as index

    // binding
    par.bind(|i| par_data(i)).unwrap();
    set.bind(|| set_data.iter().copied()).unwrap();

    // running
    let mut elements: Elements<El<'_>> = vec![].into();
    let iter = ElementsIter::<I0>::new(idx, &mut elements);
    for (i, _) in iter.enumerate() {
        let exp = par_data(set_data[i]);
        for sca in sca1 {
            assert_eq!(exp, unsafe { sca.el.get() }.eval::<V>());
        }
        let exp = par_data(set_data[2]);
        for sca in sca2 {
            assert_eq!(exp, unsafe { sca.el.get() }.eval::<V>());
        }
    }
}

// num

#[test]
fn p1_num_cint_at() {
    type I0 = SetCatInt;
    type V = ValNum;
    type El<'d> = <I0 as Set>::IdxRaw<'d>;

    // data
    let set_data = vec![0, 2, 3];
    let par_data = vec![1.0, 2.0, 7.0, 9.0];

    // modeling
    let m = Model::default();
    let set = m.cat_set_int();
    let idx = set.idx();
    let par = m.par1_num(set);
    let sca1 = [par.at(idx), par[idx]];
    let sca2 = [par.at(2), par[2]];

    // binding
    par.bind(|i| par_data[i]).unwrap();
    set.bind(|| set_data.iter().copied()).unwrap();

    // running
    let mut elements: Elements<El<'_>> = vec![].into();
    let iter = ElementsIter::<I0>::new(idx, &mut elements);
    for (i, _) in iter.enumerate() {
        let exp = par_data[set_data[i]];
        for sca in sca1 {
            assert_eq!(exp, unsafe { sca.el.get() }.eval::<V>());
        }
        let exp = par_data[set_data[2]];
        for sca in sca2 {
            assert_eq!(exp, unsafe { sca.el.get() }.eval::<V>());
        }
    }
}

#[test]
fn p1_num_cstr_at() {
    type I0 = SetCatStr;
    type V = ValNum;
    type El<'d> = <I0 as Set>::IdxRaw<'d>;

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
    let sca1 = [par.at(idx), par[idx]];
    let sca2 = [par.at(3.to_string()), par["3"], par[&set_data[2]]];

    // binding
    par.bind(|i| par_data[i]).unwrap();
    set.bind(|| &set_data).unwrap();

    // running
    let mut elements: Elements<El<'_>> = vec![].into();
    let iter = ElementsIter::<I0>::new(idx, &mut elements);
    for (i, _) in iter.enumerate() {
        let exp = par_data[&set_data[i]];
        for sca in sca1 {
            assert_eq!(exp, unsafe { sca.el.get() }.eval::<V>());
        }
        let exp = par_data[&set_data[2]];
        for sca in sca2 {
            assert_eq!(exp, unsafe { sca.el.get() }.eval::<V>());
        }
    }
}

#[test]
fn p1_num_ord_at() {
    type I0 = SetOrd;
    type V = ValNum;
    type El<'d> = <I0 as Set>::IdxRaw<'d>;

    // data
    let set_data = vec![0i64, 2, 3];
    let par_data = |i: i64| i as f64 * 2.0;

    // modeling
    let m = Model::default();
    let set = m.ord_set();
    let idx = set.idx();
    let par = m.par1_num(set);
    let sca1 = [par.at(idx), par[idx]];
    let sca2 = [par.at(2), par[2]];
    // TODO: add sca3 to test Int as index

    // binding
    par.bind(|i| par_data(i)).unwrap();
    set.bind(|| set_data.iter().copied()).unwrap();

    // running
    let mut elements: Elements<El<'_>> = vec![].into();
    let iter = ElementsIter::<I0>::new(idx, &mut elements);
    for (i, _) in iter.enumerate() {
        let exp = par_data(set_data[i]);
        for sca in sca1 {
            assert_eq!(exp, unsafe { sca.el.get() }.eval::<V>());
        }
        let exp = par_data(set_data[2]);
        for sca in sca2 {
            assert_eq!(exp, unsafe { sca.el.get() }.eval::<V>());
        }
    }
}
