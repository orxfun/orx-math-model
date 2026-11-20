use crate::vars::{Binary, Continuous, Integer};
use crate::{
    Elements, ElementsIter, Model, Set, SetCatInt, SetCatStr, SetOrd, SetSym, SymWrap, VarTyp,
};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use core::fmt::Display;

fn merge(a: impl Display, b: impl Display) -> String {
    format!("{}_{}", a, b)
}

#[test]
fn v2_at_int_cint_cstr() {
    type I0 = SetCatInt;
    type I1 = SetCatStr;
    type El0<'d> = <I0 as Set>::IdxRaw<'d>;
    type El1<'d> = <I1 as Set>::IdxRaw<'d>;

    // data
    let var_name = "x"; // TODO: var-name-fix
    let s0_data = vec![0, 1, 2, 3];
    let s1_data: Vec<_> = [0, 1, 2, 3].into_iter().map(|x| x.to_string()).collect();

    // modeling
    let m = Model::new();
    let s0 = m.cat_set_int("s0");
    let s1 = m.cat_set_str("s1");
    let i0 = s0.idx();
    let i1 = s1.idx();
    let var = m.var2::<Binary, _, _>("x", s0, s1, 0, 1);
    let sca1 = [var.at(i0, i1), var[(i0, i1)]];
    let sca2 = [var.at(i0, "3"), var[(i0, "3")]];
    let sca3 = [var.at(2, i1), var[(2, i1)]];
    let sca4 = [var.at(2, "3"), var[(2, "3")]];

    // binding
    s0.bind(|| s0_data.iter().copied()).unwrap();
    s1.bind(|| &s1_data).unwrap();

    // running
    let mut el0: Elements<El0<'_>> = vec![].into();
    let it0 = ElementsIter::<I0>::new(i0.ptr(), &mut el0);
    for (i, _) in it0.enumerate() {
        let mut el1: Elements<El1<'_>> = vec![].into();
        let it1 = ElementsIter::<I1>::new(i1.ptr(), &mut el1);
        for (j, _) in it1.enumerate() {
            let exp = format!("{var_name}_{}", merge(s0_data[i], &s1_data[j]));
            for sca in sca1 {
                assert_eq!(exp, sca.data().eval_to_string());
            }

            let exp = format!("{var_name}_{}", merge(s0_data[i], &3.to_string()));
            for sca in sca2 {
                assert_eq!(exp, sca.data().eval_to_string());
            }

            let exp = format!("{var_name}_{}", merge(2.to_string(), &s1_data[j]));
            for sca in sca3 {
                assert_eq!(exp, sca.data().eval_to_string());
            }

            let exp = format!("{var_name}_2_3");
            for sca in sca4 {
                assert_eq!(exp, sca.data().eval_to_string());
            }
        }
    }
}

#[test]
fn v2_at_int_cstr_ord() {
    type I0 = SetCatStr;
    type I1 = SetOrd;
    type El0<'d> = <I0 as Set>::IdxRaw<'d>;
    type El1<'d> = <I1 as Set>::IdxRaw<'d>;

    // data
    let var_name = "x"; // TODO: var-name-fix
    let s0_data: Vec<_> = vec![0, 1, 2].into_iter().map(|x| x.to_string()).collect();
    let s1_data = vec![0i64, 1];

    // modeling
    let m = Model::new();
    let s0 = m.cat_set_str("s0");
    let s1 = m.ord_set("s1");
    let i0 = s0.idx();
    let i1 = s1.idx();
    let var = m.var2::<Integer, _, _>("x", s0, s1, 0, 1);
    let sca1 = [var.at(i0, i1), var[(i0, i1)]];
    let sca2 = [var.at(i0, 3), var[(i0, 3)]];
    let sca3 = [var.at("2", i1), var[("2", i1)]];
    let sca4 = [var.at("2", 3), var[("2", 3)]];

    // binding
    s0.bind(|| &s0_data).unwrap();
    s1.bind(|| s1_data.iter().copied()).unwrap();

    // running
    let mut el0: Elements<El0<'_>> = vec![].into();
    let it0 = ElementsIter::<I0>::new(i0.ptr(), &mut el0);
    for (i, _) in it0.enumerate() {
        let mut el1: Elements<El1<'_>> = vec![].into();
        let it1 = ElementsIter::<I1>::new(i1.ptr(), &mut el1);
        for (j, _) in it1.enumerate() {
            let exp = format!("{var_name}_{}", merge(&s0_data[i], s1_data[j]));
            for sca in sca1 {
                assert_eq!(exp, sca.data().eval_to_string());
            }

            let exp = format!("{var_name}_{}", merge(&s0_data[i], &3.to_string()));
            for sca in sca2 {
                assert_eq!(exp, sca.data().eval_to_string());
            }

            let exp = format!("{var_name}_{}", merge(2.to_string(), &s1_data[j]));
            for sca in sca3 {
                assert_eq!(exp, sca.data().eval_to_string());
            }

            let exp = format!("{var_name}_2_3");
            for sca in sca4 {
                assert_eq!(exp, sca.data().eval_to_string());
            }
        }
    }
}

#[test]
fn v2_at_int_ord_cint() {
    type I0 = SetOrd;
    type I1 = SetCatInt;
    type El0<'d> = <I0 as Set>::IdxRaw<'d>;
    type El1<'d> = <I1 as Set>::IdxRaw<'d>;

    // data
    let var_name = "x"; // TODO: var-name-fix
    let s0_data = vec![0i64, 1, 2];
    let s1_data = vec![0, 1];

    // modeling
    let m = Model::new();
    let s0 = m.ord_set("s0");
    let s1 = m.cat_set_int("s1");
    let i0 = s0.idx();
    let i1 = s1.idx();
    let var = m.var2::<Continuous, _, _>("x", s0, s1, 0, 1);
    let sca1 = [var.at(i0, i1), var[(i0, i1)]];
    let sca2 = [var.at(i0, 3), var[(i0, 3)]];
    let sca3 = [var.at(2, i1), var[(2, i1)]];
    let sca4 = [var.at(2, 3), var[(2, 3)]];

    // binding
    s0.bind(|| s0_data.iter().copied()).unwrap();
    s1.bind(|| s1_data.iter().copied()).unwrap();

    // running
    let mut el0: Elements<El0<'_>> = vec![].into();
    let it0 = ElementsIter::<I0>::new(i0.ptr(), &mut el0);
    for (i, _) in it0.enumerate() {
        let mut el1: Elements<El1<'_>> = vec![].into();
        let it1 = ElementsIter::<I1>::new(i1.ptr(), &mut el1);
        for (j, _) in it1.enumerate() {
            let exp = format!("{var_name}_{}", merge(s0_data[i], s1_data[j]));
            for sca in sca1 {
                assert_eq!(exp, sca.data().eval_to_string());
            }

            let exp = format!("{var_name}_{}", merge(&s0_data[i], &3.to_string()));
            for sca in sca2 {
                assert_eq!(exp, sca.data().eval_to_string());
            }

            let exp = format!("{var_name}_{}", merge(2.to_string(), &s1_data[j]));
            for sca in sca3 {
                assert_eq!(exp, sca.data().eval_to_string());
            }

            let exp = format!("{var_name}_2_3");
            for sca in sca4 {
                assert_eq!(exp, sca.data().eval_to_string());
            }
        }
    }
}
