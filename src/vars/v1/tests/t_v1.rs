use crate::sets::CatInt;
use crate::{Bounds, Model, SetCatInt, SetCatStr, SetOrd, ValInt, ValNum, VarType};
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use std::collections::HashMap;

#[test]
fn v1_int_cint() {
    // modeling
    let m = Model::default();
    let set = m.cat_set_int();
    let _var = m.var1(VarType::Binary, set);
}

#[test]
fn v1_int_cstr() {
    // modeling
    let m = Model::default();
    let set = m.cat_set_str();
    let _var = m.var1(VarType::Cont(Bounds::default()), set);
}

#[test]
fn v1_int_ord() {
    // modeling
    let m = Model::default();
    let set = m.ord_set();
    let _var = m.var1(VarType::Int(Bounds::default()), set);
}
