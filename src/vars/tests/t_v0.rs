use crate::{vars::bounds::ResolvedBoundD0, Model, SetCatInt, SetSym, SymWrap, VarType};

#[test]
fn t_v0() {
    // modeling
    let m = Model::new();
    let var = m.var0("x", VarType::Binary);
    let p0 = m.par0_int("p0");
    let s = m.cat_set_int("s");
    let i = s.idx();
    let p1 = m.par1_num("p1", s);

    let [lower, upper] = [true, false].map(|x| var.data().resolve_bounds_d0(x).eval());
    assert_eq!(lower, None);
    assert_eq!(upper, None);

    // no binding
    unsafe { var.data_mut() }.set_bound_d0(true, 10);
    unsafe { var.data_mut() }.set_bound_d0(false, 12.0);
    let [lower, upper] = [true, false].map(|x| var.data().resolve_bounds_d0(x).eval());
    assert_eq!(lower, Some(10.0));
    assert_eq!(upper, Some(12.0));

    // with late binding
    unsafe { var.data_mut() }.set_bound_d0(true, p0);
    unsafe { var.data_mut() }.set_bound_d0(false, p1[i]);

    p0.bind(|| 33).unwrap();
    p1.bind(|i| i + 2).unwrap();
    i.data().write::<SetCatInt>(3);

    let [lower, upper] = [true, false].map(|x| var.data().resolve_bounds_d0(x).eval());
    assert_eq!(lower, Some(33.0));
    assert_eq!(upper, Some(5.0));
}
