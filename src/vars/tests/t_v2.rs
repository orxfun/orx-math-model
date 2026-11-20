use crate::{Model, SetCatInt, SetCatStr, SetOrd, SetSym, SymWrap, VarType};
use alloc::{string::ToString, vec::Vec};

#[test]
fn v2_int_cint_cstr() {
    type I0 = SetCatInt;
    type I1 = SetCatStr;

    let n: Vec<_> = (0..10).map(|x| x.to_string()).collect();

    // modeling
    let m = Model::new();
    let s0 = m.cat_set_int("s0");
    let s1 = m.cat_set_str("s1");
    let var = m.var2("x", VarType::Binary, s0, s1);
    let p0 = m.par0_int("p0");
    let i = s0.idx();
    let j = s1.idx();
    let p2 = m.par2_num("p1", s0, s1);

    // bounds: no binding
    let [lower, upper] = [true, false].map(|x| var.data().resolve_bounds_d2::<I0, I1>(x));
    assert_eq!(lower.eval(0, &n[2]), None);
    assert_eq!(upper.eval(2, &n[1]), None);

    // bounds: constant
    unsafe { var.data_mut() }.set_bound_d1(true, 10);
    unsafe { var.data_mut() }.set_bound_d1(false, 12.0);
    let [lower, upper] = [true, false].map(|x| var.data().resolve_bounds_d2::<I0, I1>(x));
    assert_eq!(lower.eval(0, &n[2]), Some(10.0));
    assert_eq!(upper.eval(0, &n[2]), Some(12.0));

    // bounds: with late binding - static
    unsafe { var.data_mut() }.set_bound_d2(true, p0);
    unsafe { var.data_mut() }.set_bound_d2(false, p2[(i, j)]);

    p0.bind(|| 33).unwrap();
    p2.bind(|i, j| i as f64 + j.parse::<f64>().unwrap())
        .unwrap();
    i.data().write::<I0>(3);
    j.data().write::<I1>(&n[2]);

    let [lower, upper] = [true, false].map(|x| var.data().resolve_bounds_d2::<I0, I1>(x));
    assert_eq!(lower.eval(7, &n[2]), Some(33.0));
    assert_eq!(upper.eval(9, &n[2]), Some(5.0));

    // bounds: with late binding - variable
    unsafe { var.data_mut() }.set_bound_d2(true, p2);
    unsafe { var.data_mut() }.set_bound_d2(false, p2);
    let [lower, upper] = [true, false].map(|x| var.data().resolve_bounds_d2::<I0, I1>(x));
    for i in 0..10 {
        for j in 0..10 {
            let exp = Some((i + j) as f64);
            assert_eq!(lower.eval(i, &n[j]), exp);
            assert_eq!(upper.eval(i, &n[j]), exp);
        }
    }
}

#[test]
fn v2_int_cstr_ord() {
    // modeling
    let m = Model::new();
    let s0 = m.cat_set_str("s0");
    let s1 = m.ord_set("s1");
    let _var = m.var2("x", VarType::Continuous, s0, s1);
}

#[test]
fn v2_int_ord_cint() {
    // modeling
    let m = Model::new();
    let s0 = m.ord_set("s0");
    let s1 = m.cat_set_int("s1");
    let _var = m.var2("x", VarType::Integer, s0, s1);
}
