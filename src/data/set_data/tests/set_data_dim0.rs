use crate::*;
use alloc::vec;

#[test]
fn set_data_dim0_range() {
    let m = Model::new();

    let i = m.set();

    let di = i.data(&(), |_| 0..10);
}

#[test]
fn set_data_dim0_array() {
    let m = Model::new();

    let i = m.set();

    let di = i.data(&(), |_| [3, 4, 1]);
}

#[test]
fn set_data_dim0_slice() {
    let m = Model::new();

    let i = m.set();

    let data = vec![3, 5, 1];
    let di = i.data(&data, |d| d);
}

#[test]
fn set_data_dim0_slice_filtered() {
    let m = Model::new();

    let i = m.set();

    let data = vec![3, 5, 1];
    let di = i.data(&data, |d| d.iter().filter(|x| *x % 2 == 0));
}

#[test]
fn set_data_dim0_slice_mapped() {
    let m = Model::new();

    let i = m.set();

    let data = vec![3, 5, 1];
    let di = i.data(&data, |d| d.iter().map(|x| x + 1));
}
