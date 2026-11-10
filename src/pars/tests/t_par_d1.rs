use std::collections::HashMap;

use crate::values::ValVariant;
use crate::{elements::Elements, Model};
use alloc::string::ToString;
use alloc::{vec, vec::Vec};

#[test]
fn p1_catint_to_int() {
    let data = vec![10, 11, 12, 13];

    let m = Model::new();
    let set = m.set_cat_int();
    let par = m.int_par_d1(set);

    unsafe { par.el.get_mut() }.bind(|i: usize| data[i]);

    for (i, expected) in data.iter().copied().enumerate() {
        let val = unsafe { par.el.get() }.value_as_variant(i);
        assert_eq!(val, ValVariant::Int(expected));
    }
}

// #[test]
// fn p1_catstr_to_int() {
//     let data: HashMap<_, _> = [10, 11, 12, 13]
//         .iter()
//         .enumerate()
//         .map(|(i, x)| (x.to_string(), i))
//         .collect();

//     let m = Model::new();
//     let set = m.set_cat_int();
//     let par = m.int_par_d1(set);

//     unsafe { par.el.get_mut() }.bind(|i: &str| data.get(&i).unwrap());

//     for (i, expected) in data.iter().copied().enumerate() {
//         let val = unsafe { par.el.get() }.value_as_variant(i);
//         assert_eq!(val, ValVariant::Int(expected));
//     }
// }
