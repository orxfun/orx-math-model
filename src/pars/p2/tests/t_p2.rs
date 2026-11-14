use crate::sets::CatInt;
use crate::{Model, SetCatInt, SetCatStr, SetOrd, ValInt, ValNum};
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use std::collections::HashMap;

// int

#[test]
fn p1_int_cint() {
    type I0 = SetCatInt;
    type V = ValInt;

    // data
    let values = vec![1i64, 2, 7, 9];

    // modeling
    let m = Model::default();
    let set = m.cat_set_int();
    let par = m.par1_int(set);

    // binding
    par.bind(|i| values[i]).unwrap();

    // running
    for (i, v) in values.iter().copied().enumerate() {
        let val = unsafe { par.el.get() }.at::<V, I0>(i);
        assert_eq!(v, val);
    }
}
