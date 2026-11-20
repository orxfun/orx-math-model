use crate::{Model, SetCatInt, SetSym, SymWrap, VarType};

#[test]
fn v1_int_cint() {
    type I0 = SetCatInt;

    // modeling
    let m = Model::new();
    let s = m.cat_set_int("S");
    let var = m.var1("x", VarType::Binary, s);
    let p0 = m.par0_int("p0");
    let i = s.idx();
    let p1 = m.par1_num("p1", s);

    // bounds: no binding
    let [lower, upper] = [true, false].map(|x| var.data().resolve_bounds_d1::<I0>(x));
    assert_eq!(lower.eval(0), None);
    assert_eq!(upper.eval(2), None);

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
    // modeling
    let m = Model::new();
    let set = m.cat_set_str("S");
    let _var = m.var1("x", VarType::Continuous, set);
}

#[test]
fn v1_int_ord() {
    // modeling
    let m = Model::new();
    let set = m.ord_set("S");
    let _var = m.var1("x", VarType::Integer, set);
}
