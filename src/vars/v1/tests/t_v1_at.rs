use crate::{Bounds, Elements, ElementsIter, Model, Set, SetCatInt, SetCatStr, SetOrd, VarType};
use alloc::string::ToString;
use alloc::{vec, vec::Vec};

#[test]
fn v1_at_int_cint() {
    type I0 = SetCatInt;
    type El<'d> = <I0 as Set>::IdxRaw<'d>;

    // data
    let s0_data = vec![0, 2, 3];

    // modeling
    let m = Model::default();
    let s0 = m.cat_set_int();
    let i0 = s0.idx();
    let v1 = m.var1(VarType::Binary, s0);
    let sca1 = [v1.at(i0)];
    let sca2 = [v1.at(2)];

    // binding
    s0.bind(|| s0_data.iter().copied()).unwrap();

    // running
    let mut elements: Elements<El<'_>> = vec![].into();
    let iter = ElementsIter::<I0>::new(i0, &mut elements);
    for (i, _) in iter.enumerate() {
        let exp = s0_data[i];
        for sca in sca1 {
            assert_eq!(exp, sca.data().indices_d1::<I0>());
        }
        let exp = s0_data[2];
        for sca in sca2 {
            assert_eq!(exp, sca.data().indices_d1::<I0>());
        }
    }
}

#[test]
fn v1_at_int_cstr() {
    type I0 = SetCatStr;
    type El<'d> = <I0 as Set>::IdxRaw<'d>;

    // data
    let s0_data: Vec<_> = [0, 2, 3].map(|x| x.to_string()).into_iter().collect();

    // modeling
    let m = Model::default();
    let s0 = m.cat_set_str();
    let i0 = s0.idx();
    let v1 = m.var1(VarType::Cont(Bounds::default()), s0);
    let sca1 = [v1.at(i0)];
    let sca2 = [v1.at("2")];

    // binding
    s0.bind(|| &s0_data).unwrap();

    // running
    let mut elements: Elements<El<'_>> = vec![].into();
    let iter = ElementsIter::<I0>::new(i0, &mut elements);
    for (i, _) in iter.enumerate() {
        let exp = &s0_data[i];
        for sca in sca1 {
            assert_eq!(exp, sca.data().indices_d1::<I0>());
        }
        let exp = &s0_data[2];
        for sca in sca2 {
            assert_eq!(exp, sca.data().indices_d1::<I0>());
        }
    }
}

#[test]
fn v1_at_int_ord() {
    type I0 = SetOrd;
    type El<'d> = <I0 as Set>::IdxRaw<'d>;

    // data
    let s0_data = vec![0i64, 2, 3];

    // modeling
    let m = Model::default();
    let s0 = m.ord_set();
    let i0 = s0.idx();
    let v1 = m.var1(VarType::Binary, s0);
    let sca1 = [v1.at(i0)];
    let sca2 = [v1.at(2)];

    // binding
    s0.bind(|| s0_data.iter().copied()).unwrap();

    // running
    let mut elements: Elements<El<'_>> = vec![].into();
    let iter = ElementsIter::<I0>::new(i0, &mut elements);
    for (i, _) in iter.enumerate() {
        let exp = s0_data[i];
        for sca in sca1 {
            assert_eq!(exp, sca.data().indices_d1::<I0>());
        }
        let exp = s0_data[2];
        for sca in sca2 {
            assert_eq!(exp, sca.data().indices_d1::<I0>());
        }
    }
}
