use crate::data::set_data::indices::{Depth, IndexValues, SetDepths};
use crate::data::SetDataCore;
use crate::Model;
use alloc::vec;
use alloc::vec::Vec;

#[test]
fn set_data_dim0_empty() {
    let m = Model::new();

    let i = m.set();
    let di = i.data_empty();

    let set_depths = SetDepths::new([m.set(), i, m.set()]);
    let index_values = IndexValues::new(Depth::zero().next().next().next());

    let values: Vec<_> = di.elements(&set_depths, &index_values).collect();
    assert_eq!(values, vec![]);
}

#[test]
fn set_data_dim0_range() {
    let m = Model::new();

    let i = m.set();
    let di = i.data(&(), |_| 3..7);

    let set_depths = SetDepths::new([m.set(), i, m.set()]);
    let index_values = IndexValues::new(Depth::zero().next().next().next());

    let values: Vec<_> = di.elements(&set_depths, &index_values).collect();
    assert_eq!(values, vec![3, 4, 5, 6]);
}

#[test]
fn set_data_dim0_array() {
    let m = Model::new();

    let i = m.set();
    let di = i.data(&(), |_| [3, 4, 1]);

    let set_depths = SetDepths::new([m.set(), i, m.set()]);
    let index_values = IndexValues::new(Depth::zero().next().next().next());

    let values: Vec<_> = di.elements(&set_depths, &index_values).collect();
    assert_eq!(values, vec![3, 4, 1]);
}

#[test]
fn set_data_dim0_slice() {
    let m = Model::new();

    let i = m.set();

    let data = vec![3, 5, 1];
    let di = i.data(&data, |d| d);

    let set_depths = SetDepths::new([m.set(), i, m.set()]);
    let index_values = IndexValues::new(Depth::zero().next().next().next());

    let values: Vec<_> = di.elements(&set_depths, &index_values).collect();
    assert_eq!(values, vec![3, 5, 1]);
}

#[test]
fn set_data_dim0_slice_filtered() {
    let m = Model::new();

    let i = m.set();

    let data = vec![3, 2, 5, 4, 1];
    let di = i.data(&data, |d| d.iter().filter(|x| *x % 2 == 0));

    let set_depths = SetDepths::new([m.set(), i, m.set()]);
    let index_values = IndexValues::new(Depth::zero().next().next().next());

    let values: Vec<_> = di.elements(&set_depths, &index_values).collect();
    assert_eq!(values, vec![2, 4]);
}

#[test]
fn set_data_dim0_slice_mapped() {
    let m = Model::new();

    let i = m.set();

    let data = vec![3, 5, 1];
    let di = i.data(&data, |d| d.iter().map(|x| x + 1));

    let set_depths = SetDepths::new([m.set(), i, m.set()]);
    let index_values = IndexValues::new(Depth::zero().next().next().next());

    let values: Vec<_> = di.elements(&set_depths, &index_values).collect();
    assert_eq!(values, vec![4, 6, 2]);
}
