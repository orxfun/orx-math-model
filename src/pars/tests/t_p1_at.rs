use crate::scalars::num::ValNum;
use crate::{Elements, ElementsIter, Model, Set, SetCatInt, SetCatStr, SetOrd, SymWrap};
use alloc::string::ToString;
use alloc::{vec, vec::Vec};
use std::collections::HashMap;

#[test]
fn p1_at_cint() {
    type I0 = SetCatInt;
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
            assert_eq!(exp, sca.data().eval());
        }
        let exp = par_data[set_data[2]];
        for sca in sca2 {
            assert_eq!(exp, sca.data().eval());
        }
    }
}

#[test]
fn p1_at_cstr() {
    type I0 = SetCatStr;
    type El<'d> = <I0 as Set>::IdxRaw<'d>;

    // data
    let set_data: Vec<_> = [0, 2, 3].map(|x| x.to_string()).into_iter().collect();
    let par_data: HashMap<_, _> = [1, 2, 7, 9]
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
            assert_eq!(exp, sca.data().eval_int());
        }
        let exp = par_data[&set_data[2]];
        for sca in sca2 {
            assert_eq!(exp, sca.data().eval_int());
        }
    }
}

#[test]
fn p1_at_ord() {
    type I0 = SetOrd;
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
            assert_eq!(exp, sca.data().eval());
        }
        let exp = par_data(set_data[2]);
        for sca in sca2 {
            assert_eq!(exp, sca.data().eval());
        }
    }
}
