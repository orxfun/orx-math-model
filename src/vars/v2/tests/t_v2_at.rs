use crate::sets::CatInt;
use crate::{
    Bounds, Elements, ElementsIter, Model, Set, SetCatInt, SetCatStr, SetOrd, ValInt, ValNum,
    VarType,
};
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use std::collections::HashMap;

#[test]
fn v2_at_int_cint_cstr() {
    type I0 = SetCatInt;
    type I1 = SetCatStr;
    type El0<'d> = <I0 as Set>::IdxRaw<'d>;
    type El1<'d> = <I1 as Set>::IdxRaw<'d>;

    // data
    let s0_data = vec![0, 1, 2, 3];
    let s1_data: Vec<_> = [0, 1, 2, 3].into_iter().map(|x| x.to_string()).collect();

    // modeling
    let m = Model::default();
    let s0 = m.cat_set_int();
    let s1 = m.cat_set_str();
    let i0 = s0.idx();
    let i1 = s1.idx();
    let var = m.var2(VarType::Binary, s0, s1);
    let sca1 = [var.at(i0, i1)];
    let sca2 = [var.at(i0, "3")];
    let sca3 = [var.at(2, i1)];
    let sca4 = [var.at(2, "3")];

    // binding
    s0.bind(|| s0_data.iter().copied()).unwrap();
    s1.bind(|| &s1_data).unwrap();

    // running
    let mut el0: Elements<El0<'_>> = vec![].into();
    let it0 = ElementsIter::<I0>::new(i0, &mut el0);
    for (i, _) in it0.enumerate() {
        let mut el1: Elements<El1<'_>> = vec![].into();
        let it1 = ElementsIter::<I1>::new(i1, &mut el1);
        for (j, _) in it1.enumerate() {
            let exp = (s0_data[i], &s1_data[j]);
            for sca in sca1 {
                assert_eq!(exp, unsafe { sca.el.get() }.indices_d2::<I0, I1>());
            }

            let exp = (s0_data[i], &s1_data[3]);
            for sca in sca2 {
                assert_eq!(exp, unsafe { sca.el.get() }.indices_d2::<I0, I1>());
            }

            let exp = (s0_data[2], &s1_data[j]);
            for sca in sca3 {
                assert_eq!(exp, unsafe { sca.el.get() }.indices_d2::<I0, I1>());
            }

            let exp = (s0_data[2], &s1_data[3]);
            for sca in sca4 {
                assert_eq!(exp, unsafe { sca.el.get() }.indices_d2::<I0, I1>());
            }
        }
    }
}

#[test]
fn v2_at_int_cstr_ord() {
    // modeling
    let m = Model::default();
    let s0 = m.cat_set_str();
    let s1 = m.ord_set();
    let _var = m.var2(VarType::Cont(Bounds::default()), s0, s1);
}

#[test]
fn v2_at_int_ord_cint() {
    // modeling
    let m = Model::default();
    let s0 = m.ord_set();
    let s1 = m.cat_set_int();
    let _var = m.var2(VarType::Int(Bounds::default()), s0, s1);
}

#[test]
fn abc() {
    type I0 = SetCatInt;
    type I1 = SetCatStr;
    type El0<'d> = <I0 as Set>::IdxRaw<'d>;
    type El1<'d> = <I1 as Set>::IdxRaw<'d>;

    // data
    let s0_data = vec![0, 1, 2, 3];
    let s1_data: Vec<_> = [0, 1, 2, 3].into_iter().map(|x| x.to_string()).collect();

    // modeling
    let m = Model::default();
    let s0 = m.cat_set_int();
    let s1 = m.cat_set_str();
    let i0 = s0.idx();
    let i1 = s1.idx();
    let var = m.var2(VarType::Binary, s0, s1);
    let sca1 = [var.at(i0, i1)];
    let sca2 = [var.at(i0, "3")];
    let sca3 = [var.at(2, i1)];
    let sca4 = [var.at(2, "3")];

    // binding
    s0.bind(|| s0_data.iter().copied()).unwrap();
    s1.bind(|| &s1_data).unwrap();

    // running
    let mut el0: Elements<El0<'_>> = vec![].into();
    let it0 = ElementsIter::<I0>::new(i0, &mut el0);
    for (i, _) in it0.enumerate() {
        let mut el1: Elements<El1<'_>> = vec![].into();
        let it1 = ElementsIter::<I1>::new(i1, &mut el1);
        for (j, _) in it1.enumerate() {
            let exp = (s0_data[i], &s1_data[j]);
            for sca in sca1 {
                assert_eq!(exp, unsafe { sca.el.get() }.indices_d2::<I0, I1>());
            }

            let exp = (s0_data[i], &s1_data[3]);
            for sca in sca2 {
                assert_eq!(exp, unsafe { sca.el.get() }.indices_d2::<I0, I1>());
            }

            let exp = (s0_data[2], &s1_data[j]);
            for sca in sca3 {
                assert_eq!(exp, unsafe { sca.el.get() }.indices_d2::<I0, I1>());
            }

            let exp = (s0_data[2], &s1_data[3]);
            for sca in sca4 {
                assert_eq!(exp, unsafe { sca.el.get() }.indices_d2::<I0, I1>());
            }
        }
    }
}
