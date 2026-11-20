use crate::vars::Binary;
use crate::{
    Elements, ElementsIter, Model, Set, SetCatInt, SetCatStr, SetOrd, SetSym, SymWrap, VarTyp,
};
use alloc::format;
use alloc::string::ToString;
use alloc::{vec, vec::Vec};

#[test]
fn v1_at_int_cint() {
    type I0 = SetCatInt;
    type El<'d> = <I0 as Set>::IdxRaw<'d>;

    // data
    let var_name = "x"; // TODO: var-name-fix
    let s0_data = vec![0, 2, 3];

    // modeling
    let m = Model::new();
    let s0 = m.cat_set_int("s0");
    let i0 = s0.idx();
    let v1 = m.var1::<Binary, _>("x", s0, 0, 1);
    let sca1 = [v1.at(i0), v1[i0]];
    let sca2 = [v1.at(2), v1[2]];

    // binding
    s0.bind(|| s0_data.iter().copied()).unwrap();

    // running
    let mut elements: Elements<El<'_>> = vec![].into();
    let iter = ElementsIter::<I0>::new(i0.ptr(), &mut elements);
    for (i, _) in iter.enumerate() {
        let exp = format!("{}_{}", var_name, s0_data[i].to_string());
        for sca in sca1 {
            assert_eq!(exp, sca.data().eval_to_string());
        }
        let exp = format!("{}_2", var_name);
        for sca in sca2 {
            assert_eq!(exp, sca.data().eval_to_string());
        }
    }
}

#[test]
fn v1_at_int_cstr() {
    type I0 = SetCatStr;
    type El<'d> = <I0 as Set>::IdxRaw<'d>;

    // data
    let var_name = "x"; // TODO: var-name-fix
    let s0_data: Vec<_> = [0, 2, 3].map(|x| x.to_string()).into_iter().collect();

    // modeling
    let m = Model::new();
    let s0 = m.cat_set_str("s0");
    let i0 = s0.idx();
    let v1 = m.var1::<Binary, _>("x", s0, 0, 1);
    let sca1 = [v1.at(i0), v1[i0]];
    let sca2 = [v1.at("2"), v1["2"]];

    // binding
    s0.bind(|| &s0_data).unwrap();

    // running
    let mut elements: Elements<El<'_>> = vec![].into();
    let iter = ElementsIter::<I0>::new(i0.ptr(), &mut elements);
    for (i, _) in iter.enumerate() {
        let exp = format!("{}_{}", var_name, s0_data[i].to_string());
        for sca in sca1 {
            assert_eq!(exp, sca.data().eval_to_string());
        }
        let exp = format!("{}_2", var_name);
        for sca in sca2 {
            assert_eq!(exp, sca.data().eval_to_string());
        }
    }
}

#[test]
fn v1_at_int_ord() {
    type I0 = SetOrd;
    type El<'d> = <I0 as Set>::IdxRaw<'d>;

    // data
    let var_name = "x"; // TODO: var-name-fix
    let s0_data = vec![0i64, 2, 3];

    // modeling
    let m = Model::new();
    let s0 = m.ord_set("s0");
    let i0 = s0.idx();
    let v1 = m.var1::<Binary, _>("x", s0, 0, 1);
    let sca1 = [v1.at(i0), v1[i0]];
    let sca2 = [v1.at(2), v1[2]];

    // binding
    s0.bind(|| s0_data.iter().copied()).unwrap();

    // running
    let mut elements: Elements<El<'_>> = vec![].into();
    let iter = ElementsIter::<I0>::new(i0.ptr(), &mut elements);
    for (i, _) in iter.enumerate() {
        let exp = format!("{}_{}", var_name, s0_data[i].to_string());
        for sca in sca1 {
            assert_eq!(exp, sca.data().eval_to_string());
        }
        let exp = format!("{}_2", var_name);
        for sca in sca2 {
            assert_eq!(exp, sca.data().eval_to_string());
        }
    }
}
