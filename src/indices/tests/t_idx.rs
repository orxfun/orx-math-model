use crate::{elements::Elements, Model};
use alloc::string::ToString;
use alloc::{vec, vec::Vec};

#[test]
fn idx_cat_int() {
    let values = vec![1, 2, 7, 9];

    let m = Model::new();
    let set = m.set_cat_int();
    let idx = set.idx();

    let mut elements = Elements::from(vec![]);
    unsafe { set.el.get() }.as_cat_int().fill(&mut elements);
    assert_eq!(elements.as_slice(), &[]);
}
