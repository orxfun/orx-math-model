use crate::data::set_data::indices::{Depth, IndexValues, SetDepths};
use crate::data::SetData;
use crate::*;
use alloc::vec;
use alloc::vec::Vec;

#[test]
fn set_data_dim1_jagged() {
    let m = Model::new();

    let i = m.set();
    let j = set_of(i);

    let data1 = vec![vec![], vec![1, 2, 3], vec![3, 5]];
    let dj = j.data(&data1, |d, i| &d[i]);

    let set_depths = SetDepths::new([i.core(), j.core(), m.set().core()]);
    let depth_i = Depth::zero();
    let mut index_values = IndexValues::new(Depth::zero().next().next().next());

    index_values[depth_i] = 0;
    let values: Vec<_> = dj.elements(&set_depths, &index_values).collect();
    assert_eq!(values, vec![]);

    index_values[depth_i] = 1;
    let values: Vec<_> = dj.elements(&set_depths, &index_values).collect();
    assert_eq!(values, vec![1, 2, 3]);

    index_values[depth_i] = 2;
    let values: Vec<_> = dj.elements(&set_depths, &index_values).collect();
    assert_eq!(values, vec![3, 5]);
}

#[test]
fn set_data_dim1_next() {
    let m = Model::new();

    let i = m.set();
    let j = set_of(i);

    let card_i = 5;
    let dj = j.data(&card_i, |c, i| (i < c - 1).then_some(i + 1));

    let set_depths = SetDepths::new([m.set().core(), i.core(), j.core()]);
    let depth_i = Depth::zero().next();
    let mut index_values = IndexValues::new(Depth::zero().next().next().next());

    index_values[depth_i] = 0;
    let values: Vec<_> = dj.elements(&set_depths, &index_values).collect();
    assert_eq!(values, vec![1]);

    index_values[depth_i] = 1;
    let values: Vec<_> = dj.elements(&set_depths, &index_values).collect();
    assert_eq!(values, vec![2]);

    index_values[depth_i] = 3;
    let values: Vec<_> = dj.elements(&set_depths, &index_values).collect();
    assert_eq!(values, vec![4]);

    index_values[depth_i] = 4;
    let values: Vec<_> = dj.elements(&set_depths, &index_values).collect();
    assert_eq!(values, vec![]);
}

#[test]
fn set_data_dim1_up_to() {
    let m = Model::new();

    let i = m.set();
    let j = set_of(i);

    let dj = j.data(&(), |_, i| 0..i);

    let set_depths = SetDepths::new([m.set().core(), i.core(), j.core()]);
    let depth_i = Depth::zero().next();
    let mut index_values = IndexValues::new(Depth::zero().next().next().next());

    index_values[depth_i] = 0;
    let values: Vec<_> = dj.elements(&set_depths, &index_values).collect();
    assert_eq!(values, vec![]);

    index_values[depth_i] = 1;
    let values: Vec<_> = dj.elements(&set_depths, &index_values).collect();
    assert_eq!(values, vec![0]);

    index_values[depth_i] = 2;
    let values: Vec<_> = dj.elements(&set_depths, &index_values).collect();
    assert_eq!(values, vec![0, 1]);

    index_values[depth_i] = 3;
    let values: Vec<_> = dj.elements(&set_depths, &index_values).collect();
    assert_eq!(values, vec![0, 1, 2]);
}

#[test]
fn set_data_dim1_from() {
    let m = Model::new();

    let i = m.set();
    let j = set_of(i);

    let card_i = 5;
    let dj = j.data(&card_i, |c, i| (i + 1)..*c);

    let set_depths = SetDepths::new([m.set().core(), i.core(), j.core()]);
    let depth_i = Depth::zero().next();
    let mut index_values = IndexValues::new(Depth::zero().next().next().next());

    index_values[depth_i] = 0;
    let values: Vec<_> = dj.elements(&set_depths, &index_values).collect();
    assert_eq!(values, vec![1, 2, 3, 4]);

    index_values[depth_i] = 1;
    let values: Vec<_> = dj.elements(&set_depths, &index_values).collect();
    assert_eq!(values, vec![2, 3, 4]);

    index_values[depth_i] = 2;
    let values: Vec<_> = dj.elements(&set_depths, &index_values).collect();
    assert_eq!(values, vec![3, 4]);

    index_values[depth_i] = 3;
    let values: Vec<_> = dj.elements(&set_depths, &index_values).collect();
    assert_eq!(values, vec![4]);

    index_values[depth_i] = 4;
    let values: Vec<_> = dj.elements(&set_depths, &index_values).collect();
    assert_eq!(values, vec![]);
}
