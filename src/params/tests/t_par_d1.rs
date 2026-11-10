// use crate::{elements::Elements, Model};
// use alloc::string::ToString;
// use alloc::{vec, vec::Vec};

// #[test]
// fn idx_cat_int_2int() {
//     let values = vec![1, 2, 7, 9];
//     let mut elements = Elements::from(vec![]);

//     let m = Model::new();
//     let set = m.set_cat();
//     let par = m.par_d1_int(set);

//     let idx = set.idx();
//     // let sca = par.at

//     let mut iter = idx.iter(&mut elements);
//     assert!(iter.next().is_none());

//     unsafe { set.el.get_mut() }
//         .as_cat_int_mut()
//         .bind(|| values.iter().copied());
//     let iter = idx.iter(&mut elements);
//     for (i, _) in iter.enumerate() {
//         let val = unsafe { idx.el.get() }.val_ptr_int().val();
//         assert_eq!(val, values[i]);
//     }
// }
