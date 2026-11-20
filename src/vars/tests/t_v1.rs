use crate::{vars::Binary, Model, SetCatInt, SetCatStr, SetOrd, SetSym, SymWrap, VarTyp};
use alloc::{string::ToString, vec::Vec};

#[test]
fn v1_int_cint() {
    type I0 = SetCatInt;

    // modeling
    let m = Model::new();
    let s = m.cat_set_int("S");
    let var = m.var1::<Binary, _>("x", s, 0, 1);
    let p0 = m.par0_int("p0");
    let i = s.idx();
    let p1 = m.par1_num("p1", s);

    // bounds: constant
    unsafe { var.data_mut() }.set_bound_d1(true, 10);
    unsafe { var.data_mut() }.set_bound_d1(false, 12.0);
    let [lower, upper] = [true, false].map(|x| var.data().resolve_bounds_d1::<I0>(x));
    assert_eq!(lower.eval(1), Some(10.0));
    assert_eq!(upper.eval(7), Some(12.0));

    // bounds: with late binding - static
    unsafe { var.data_mut() }.set_bound_d1(true, p0);
    unsafe { var.data_mut() }.set_bound_d1(false, p1[i]);

    p0.bind(|| 33).unwrap();
    p1.bind(|i| i + 2).unwrap();
    i.data().write::<I0>(3);

    let [lower, upper] = [true, false].map(|x| var.data().resolve_bounds_d1::<I0>(x));
    assert_eq!(lower.eval(23), Some(33.0));
    assert_eq!(upper.eval(32), Some(5.0));

    // bounds: with late binding - variable
    unsafe { var.data_mut() }.set_bound_d1(true, p1);
    unsafe { var.data_mut() }.set_bound_d1(false, p1);
    let [lower, upper] = [true, false].map(|x| var.data().resolve_bounds_d1::<I0>(x));
    for i in 0..24 {
        let exp = Some((i + 2) as f64);
        assert_eq!(lower.eval(i), exp);
        assert_eq!(upper.eval(i), exp);
    }
}

#[test]
fn v1_int_cstr() {
    type I0 = SetCatStr;

    let numbers: Vec<_> = (0..30).map(|x| x.to_string()).collect();

    // modeling
    let m = Model::new();
    let s = m.cat_set_str("S");
    let var = m.var1::<Binary, _>("x", s, 0, 1);
    let p0 = m.par0_int("p0");
    let i = s.idx();
    let p1 = m.par1_num("p1", s);

    // bounds: constant
    unsafe { var.data_mut() }.set_bound_d1(true, 10);
    unsafe { var.data_mut() }.set_bound_d1(false, 12.0);
    let [lower, upper] = [true, false].map(|x| var.data().resolve_bounds_d1::<I0>(x));
    assert_eq!(lower.eval(&numbers[7]), Some(10.0));
    assert_eq!(upper.eval(&numbers[7]), Some(12.0));

    // bounds: with late binding - static
    unsafe { var.data_mut() }.set_bound_d1(true, p0);
    unsafe { var.data_mut() }.set_bound_d1(false, p1[i]);

    p0.bind(|| 33).unwrap();
    p1.bind(|i| i.parse::<f64>().unwrap() + 2.0).unwrap();
    i.data().write::<I0>(&numbers[3]);

    let [lower, upper] = [true, false].map(|x| var.data().resolve_bounds_d1::<I0>(x));
    assert_eq!(lower.eval(&numbers[23]), Some(33.0));
    assert_eq!(upper.eval(&numbers[12]), Some(5.0));

    // bounds: with late binding - variable
    unsafe { var.data_mut() }.set_bound_d1(true, p1);
    unsafe { var.data_mut() }.set_bound_d1(false, p1);
    let [lower, upper] = [true, false].map(|x| var.data().resolve_bounds_d1::<I0>(x));
    for i in 0..24 {
        let exp = Some((i + 2) as f64);
        assert_eq!(lower.eval(&numbers[i]), exp);
        assert_eq!(upper.eval(&numbers[i]), exp);
    }
}

#[test]
fn v1_int_ord() {
    type I0 = SetOrd;

    // modeling
    let m = Model::new();
    let s = m.ord_set("S");
    let var = m.var1::<Binary, _>("x", s, 0, 1);
    let p0 = m.par0_int("p0");
    let i = s.idx();
    let p1 = m.par1_num("p1", s);

    // bounds: constant
    unsafe { var.data_mut() }.set_bound_d1(true, 10);
    unsafe { var.data_mut() }.set_bound_d1(false, 12.0);
    let [lower, upper] = [true, false].map(|x| var.data().resolve_bounds_d1::<I0>(x));
    assert_eq!(lower.eval(1), Some(10.0));
    assert_eq!(upper.eval(7), Some(12.0));

    // bounds: with late binding - static
    unsafe { var.data_mut() }.set_bound_d1(true, p0);
    unsafe { var.data_mut() }.set_bound_d1(false, p1[i]);

    p0.bind(|| 33).unwrap();
    p1.bind(|i| i + 2).unwrap();
    i.data().write::<I0>(3);

    let [lower, upper] = [true, false].map(|x| var.data().resolve_bounds_d1::<I0>(x));
    assert_eq!(lower.eval(23), Some(33.0));
    assert_eq!(upper.eval(32), Some(5.0));

    // bounds: with late binding - variable
    unsafe { var.data_mut() }.set_bound_d1(true, p1);
    unsafe { var.data_mut() }.set_bound_d1(false, p1);
    let [lower, upper] = [true, false].map(|x| var.data().resolve_bounds_d1::<I0>(x));
    for i in 0..24 {
        let exp = Some((i + 2) as f64);
        assert_eq!(lower.eval(i), exp);
        assert_eq!(upper.eval(i), exp);
    }
}
