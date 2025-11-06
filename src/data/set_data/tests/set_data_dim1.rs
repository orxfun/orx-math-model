use crate::*;
use alloc::vec;
use alloc::vec::Vec;

#[test]
fn set_data_dim1_jagged() {
    // model

    let m = Model::new();

    let i = m.set().key("i");
    let j = set_of([i]);

    let data1 = vec![vec![], vec![1, 2, 3], vec![3, 5]];
    let dj = j.data(&data1, |d, i| d[i].iter().copied());
}
