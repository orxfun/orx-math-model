use crate::data::set_data::indices::{Depth, IndexValues, SetDepths};
use crate::data::{SetAndData, SetGen};
use crate::Model;
use alloc::vec;
use alloc::vec::Vec;

#[test]
fn set_data_dim0_range() {
    let m = Model::new();

    let i = m.set();
    let j = m.set();
    let k = m.set();

    let set_depths = SetDepths::new([j, i, k]);
    let index_values = IndexValues::new(Depth::zero().next().next().next());

    let di = i.data(&(), |_| 0..10);
    let values: Vec<_> = di
        .set_gen()
        .elements(di.set(), &set_depths, &index_values)
        .collect();
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
