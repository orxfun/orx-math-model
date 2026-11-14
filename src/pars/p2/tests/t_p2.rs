use crate::sets::CatInt;
use crate::{Model, SetCatInt, SetCatStr, SetOrd, ValInt, ValNum};
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};
use std::collections::HashMap;

// int

#[test]
fn p2_int_cint_cstr() {
    type V = ValInt;
    type I0 = SetCatInt;
    type I1 = SetCatStr;

    // data
    let values = |i0: usize| {
        [1, 2, 7, 9]
            .into_iter()
            .enumerate()
            .map(|(i, v)| (i.to_string(), (10 * i0 + v) as i64))
            .collect::<HashMap<_, _>>()
    };
    let par_data: Vec<HashMap<String, i64>> = (0..4).map(|i0| values(i0)).collect();

    // modeling
    let m = Model::default();
    let s1 = m.cat_set_int();
    let s2 = m.cat_set_str();
    let par = m.par2_int(s1, s2);

    // binding
    par.bind(|i, j| par_data[i][j]).unwrap();

    // running
    for (i, map) in par_data.iter().enumerate() {
        for (j, exp) in map.iter() {
            let val = unsafe { par.el.get() }.at::<V, I0, I1>(i, j);
            assert_eq!(*exp, val);
        }
    }
}
