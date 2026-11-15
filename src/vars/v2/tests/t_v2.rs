use crate::{Bounds, Model, VarType};

#[test]
fn v2_int_cint_cstr() {
    // modeling
    let m = Model::default();
    let s0 = m.cat_set_int();
    let s1 = m.cat_set_str();
    let _var = m.var2(VarType::Binary, s0, s1);
}

#[test]
fn v2_int_cstr_ord() {
    // modeling
    let m = Model::default();
    let s0 = m.cat_set_str();
    let s1 = m.ord_set();
    let _var = m.var2(VarType::Cont(Bounds::default()), s0, s1);
}

#[test]
fn v2_int_ord_cint() {
    // modeling
    let m = Model::default();
    let s0 = m.ord_set();
    let s1 = m.cat_set_int();
    let _var = m.var2(VarType::Int(Bounds::default()), s0, s1);
}
