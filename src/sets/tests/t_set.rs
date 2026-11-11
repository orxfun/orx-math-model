use crate::{elements::Elements, Model};
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};

#[test]
fn set_cat_int() {
    let values = vec![1usize, 2, 7, 9];

    // let m = Model::default();
    // let set = m.cat_set_int();
    // let mut elements: Elements<usize> = vec![].into();
    // unsafe { set.el.get_mut() }.bind_from(|| values.iter().copied());

    // unsafe { set.el.get() }.fill(&mut elements);
    // assert_eq!(elements.as_slice(), &[1, 2, 7, 9]);
}

// #[test]
// fn set_cat_str() {
//     let values: Vec<_> = [1, 2, 7, 9].map(|x| x.to_string()).into_iter().collect();

//     let m = Model::new();
//     let set = m.set_cat_str();
//     let mut elements: Elements<&String> = vec![].into();

//     unsafe { set.el.get_mut() }.bind_from(|| values.iter());
//     unsafe { set.el.get() }.fill(&mut elements);
//     assert_eq!(elements.as_slice(), &["1", "2", "7", "9"]);
// }

// #[test]
// fn set_ord() {
//     let values = vec![1i64, 2, 7, 9];

//     let m = Model::new();
//     let set = m.set_ord();
//     let mut elements: Elements<i64> = vec![].into();

//     unsafe { set.el.get_mut() }.bind_from(|| values.iter().copied());
//     unsafe { set.el.get() }.fill(&mut elements);
//     assert_eq!(elements.as_slice(), &[1, 2, 7, 9]);
// }
