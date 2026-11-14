use crate::sets::CatInt;
use crate::{Elements, ElementsIter, Model, Set, SetCatInt, SetCatStr, SetOrd, ValInt, ValNum};
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use std::collections::HashMap;

// int

#[test]
fn p2_at_int_cint_cstr() {
    type V = ValInt;
    type I0 = SetCatInt;
    type I1 = SetCatStr;
    type El0<'d> = <I0 as Set>::IdxRaw<'d>;
    type El1<'d> = <I1 as Set>::IdxRaw<'d>;

    // data
    let values = |i0: usize| {
        [1, 2, 7, 9]
            .into_iter()
            .enumerate()
            .map(|(i, v)| (i.to_string(), (10 * i0 + v) as i64))
            .collect::<HashMap<_, _>>()
    };
    let par_data: Vec<HashMap<String, i64>> = (0..4).map(|i0| values(i0)).collect();
    let s0_data = vec![0, 1, 2, 3];
    let s1_data: Vec<_> = [0, 1, 2, 3].into_iter().map(|x| x.to_string()).collect();

    // modeling
    let m = Model::default();
    let s0 = m.cat_set_int();
    let s1 = m.cat_set_str();
    let i0 = s0.idx();
    let i1 = s1.idx();
    let par = m.par2_int(s0, s1);
    let sca1 = [par.at(i0, i1), par[(i0, i1)]];
    let sca2 = [par.at(i0, 2.to_string()), par[(i0, 2.to_string())]];
    let sca3 = [par.at(2, i1), par[(2, i1)]];
    let sca4 = [par.at(2, 2.to_string()), par[(2, 2.to_string())]];

    // binding
    par.bind(|i, j| par_data[i][j]).unwrap();
    s0.bind(|| s0_data.iter().copied()).unwrap();
    s1.bind(|| &s1_data).unwrap();

    // running
    let mut el0: Elements<El0<'_>> = vec![].into();
    let it0 = ElementsIter::<I0>::new(i0, &mut el0);
    for (i, _) in it0.enumerate() {
        let mut el1: Elements<El1<'_>> = vec![].into();
        let it1 = ElementsIter::<I1>::new(i1, &mut el1);
        for (j, _) in it1.enumerate() {
            let exp = par_data[i][&j.to_string()];
            for sca in sca1 {
                assert_eq!(exp, unsafe { sca.el.get() }.eval::<V>());
            }

            let exp = par_data[i][&2.to_string()];
            for sca in sca2 {
                assert_eq!(exp, unsafe { sca.el.get() }.eval::<V>());
            }

            let exp = par_data[2][&j.to_string()];
            for sca in sca3 {
                assert_eq!(exp, unsafe { sca.el.get() }.eval::<V>());
            }

            let exp = par_data[2][&2.to_string()];
            for sca in sca4 {
                assert_eq!(exp, unsafe { sca.el.get() }.eval::<V>());
            }
        }
    }
}

#[test]
fn p2_at_int_cstr_ord() {
    type V = ValInt;
    type I0 = SetCatStr;
    type I1 = SetOrd;
    type El0<'d> = <I0 as Set>::IdxRaw<'d>;
    type El1<'d> = <I1 as Set>::IdxRaw<'d>;

    // data
    let values = vec![vec![0i64, 1], vec![2, 3], vec![4, 5]];
    let par_data: HashMap<String, Vec<i64>> = values
        .into_iter()
        .enumerate()
        .map(|(i, v)| (i.to_string(), v))
        .collect();
    let s0_data: Vec<_> = vec![0, 1, 2].into_iter().map(|x| x.to_string()).collect();
    let s1_data = vec![0i64, 1];

    // modeling
    let m = Model::default();
    let s0 = m.cat_set_str();
    let s1 = m.ord_set();
    let i0 = s0.idx();
    let i1 = s1.idx();
    let par = m.par2_int(s0, s1);
    let sca1 = [par.at(i0, i1), par[(i0, i1)]];
    let sca2 = [par.at(2.to_string(), i1), par[(2.to_string(), i1)]];
    let sca3 = [par.at(i0, 2), par[(i0, 2)]];
    let sca4 = [par.at(2.to_string(), 2), par[(2.to_string(), 2)]];

    // binding
    par.bind(|i, j| par_data[i][j as usize]).unwrap();
    s0.bind(|| &s0_data).unwrap();
    s1.bind(|| s1_data.iter().copied()).unwrap();

    // running
    let mut el0: Elements<El0<'_>> = vec![].into();
    let it0 = ElementsIter::<I0>::new(i0, &mut el0);
    for (i, _) in it0.enumerate() {
        let mut el1: Elements<El1<'_>> = vec![].into();
        let it1 = ElementsIter::<I1>::new(i1, &mut el1);
        for (j, _) in it1.enumerate() {
            let exp = par_data[&i.to_string()][j];
            for sca in sca1 {
                assert_eq!(exp, unsafe { sca.el.get() }.eval::<V>());
            }

            let exp = par_data[&2.to_string()][j];
            for sca in sca2 {
                assert_eq!(exp, unsafe { sca.el.get() }.eval::<V>());
            }

            let exp = par_data[&i.to_string()][2];
            for sca in sca3 {
                assert_eq!(exp, unsafe { sca.el.get() }.eval::<V>());
            }

            let exp = par_data[&2.to_string()][2];
            for sca in sca4 {
                assert_eq!(exp, unsafe { sca.el.get() }.eval::<V>());
            }
        }
    }
}

#[test]
fn p2_at_int_ord_cint() {
    type V = ValInt;
    type I0 = SetOrd;
    type I1 = SetCatInt;
    type El0<'d> = <I0 as Set>::IdxRaw<'d>;
    type El1<'d> = <I1 as Set>::IdxRaw<'d>;

    // data
    let par_data = vec![vec![0i64, 1], vec![2, 3], vec![4, 5]];
    let s0_data = vec![0i64, 1, 2];
    let s1_data = vec![0, 1];

    // modeling
    let m = Model::default();
    let s0 = m.ord_set();
    let s1 = m.cat_set_int();
    let i0 = s0.idx();
    let i1 = s1.idx();
    let par = m.par2_int(s0, s1);
    let sca1 = [par.at(i0, i1), par[(i0, i1)]];
    let sca2 = [par.at(i0, 2), par[(i0, 2)]];
    let sca3 = [par.at(2, i1), par[(2, i1)]];
    let sca4 = [par.at(2, 2), par[(2, 2)]];

    // binding
    par.bind(|i, j| par_data[i as usize][j]).unwrap();
    s0.bind(|| s0_data.iter().copied()).unwrap();
    s1.bind(|| s1_data.iter().copied()).unwrap();

    // running
    let mut el0: Elements<El0<'_>> = vec![].into();
    let it0 = ElementsIter::<I0>::new(i0, &mut el0);
    for (i, _) in it0.enumerate() {
        let mut el1: Elements<El1<'_>> = vec![].into();
        let it1 = ElementsIter::<I1>::new(i1, &mut el1);
        for (j, _) in it1.enumerate() {
            let exp = par_data[i][j];
            for sca in sca1 {
                assert_eq!(exp, unsafe { sca.el.get() }.eval::<V>());
            }

            let exp = par_data[i][2];
            for sca in sca2 {
                assert_eq!(exp, unsafe { sca.el.get() }.eval::<V>());
            }

            let exp = par_data[2][j];
            for sca in sca3 {
                assert_eq!(exp, unsafe { sca.el.get() }.eval::<V>());
            }

            let exp = par_data[2][2];
            for sca in sca4 {
                assert_eq!(exp, unsafe { sca.el.get() }.eval::<V>());
            }
        }
    }
}
