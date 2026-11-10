use crate::values::ValVariant;
use crate::{elements::Elements, Model};
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use std::collections::HashMap;
use std::println;

// int

#[test]
fn p1_catint_to_int() {
    let data = vec![10, 11, 12, 13];

    let m = Model::new();
    let set = m.set_cat_int();
    let idx = set.idx();
    let par = m.int_par_d1(set);

    unsafe { par.el.get_mut() }.bind(|i: usize| data[i]);
}
