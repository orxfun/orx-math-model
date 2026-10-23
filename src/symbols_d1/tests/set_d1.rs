use crate::symbols::{Elements, Set, SetData};
use crate::Model;
use alloc::vec;

#[test]
fn set_get_set_d1() {
    let data = vec![vec![0], vec![1, 2, 3], vec![4, 5], vec![6, 7, 8, 9]];

    let m = Model::new();

    let s0 = m.set();
    let s1 = m.set_d1(s0);
}
