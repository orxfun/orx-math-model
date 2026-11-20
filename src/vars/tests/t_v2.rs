use crate::{Bounds, Model, VarType};

#[test]
fn v2_int_cint_cstr() {
    // modeling
    let m = Model::new();
    let s0 = m.cat_set_int("s0");
    let s1 = m.cat_set_str("s1");
    let _var = m.var2(VarType::Binary, s0, s1);
}

#[test]
fn v2_int_cstr_ord() {
    // modeling
    let m = Model::new();
    let s0 = m.cat_set_str("s0");
    let s1 = m.ord_set("s1");
    let _var = m.var2(VarType::Cont(Bounds::default()), s0, s1);
}

#[test]
fn v2_int_ord_cint() {
    // modeling
    let m = Model::new();
    let s0 = m.ord_set("s0");
    let s1 = m.cat_set_int("s1");
    let _var = m.var2(VarType::Int(Bounds::default()), s0, s1);
}
