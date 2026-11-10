use crate::values::ValVariant;
use crate::{elements::Elements, Model};
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use std::collections::HashMap;
use std::println;

// int

// #[test]
fn p1_catint_to_int_at() {
    let set_data = vec![0usize, 1, 2, 3];
    let par_data = vec![10, 11, 12, 13];

    let m = Model::new();
    let set = m.set_cat_int();
    let idx = set.idx();
    let par = m.int_par_d1(set);
    let sca = par[idx];

    unsafe { set.el.get_mut() }.bind_from(|| set_data.iter().copied());
    unsafe { par.el.get_mut() }.bind(|i: usize| par_data[i]);
    let mut elements: Elements<usize> = vec![].into();

    let iter = unsafe { idx.el.get() }.iter(&mut elements);
    for (i, _) in iter.enumerate() {
        let val = unsafe { sca.el.get() }.value_as_variant();
        assert_eq!(val, ValVariant::Int(par_data[i]));
    }
}
