use crate::*;
use alloc::vec;
use alloc::vec::Vec;

#[test]
fn set_data_dim0_range() {
    // model

    let m = Model::new();

    let i = m.set();

    let di = i.data(&(), |_| 0..10);
}

#[test]
fn set_data_dim0_slice() {
    // model

    let m = Model::new();

    let i = m.set();

    let data = vec![3, 5, 1];
    let di = i.data(&data, |d| d.iter().copied());
}
