use crate::data::set_data::indices::{Depth, IndexValues, SetDepths};
use crate::data::SetAndData;
use crate::*;
use alloc::vec;
use alloc::vec::Vec;

#[test]
fn set_data_dim1_jagged() {
    let m = Model::new();

    let i = m.set();
    let j = set_of([i]);

    let data1 = vec![vec![], vec![1, 2, 3], vec![3, 5]];
    let dj = j.data(&data1, |d, i| &d[i]);

    let set_depths = SetDepths::new([i.core(), j.core(), m.set().core()]);
    let mut index_values = IndexValues::new(Depth::zero().next().next().next());

    let depth_i = Depth::zero();

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
