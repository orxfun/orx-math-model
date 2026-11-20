use crate::{Bounds, Model, VarType};

#[test]
fn v1_int_cint() {
    // modeling
    let m = Model::new();
    let set = m.cat_set_int("S");
    let _var = m.var1(VarType::Binary, set);
}

#[test]
fn v1_int_cstr() {
    // modeling
    let m = Model::new();
    let set = m.cat_set_str("S");
    let _var = m.var1(VarType::Cont(Bounds::default()), set);
}

#[test]
fn v1_int_ord() {
    // modeling
    let m = Model::new();
    let set = m.ord_set("S");
    let _var = m.var1(VarType::Int(Bounds::default()), set);
}
