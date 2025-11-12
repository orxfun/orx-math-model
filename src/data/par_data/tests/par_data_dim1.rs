use crate::data::par_data::par_and_data::ParAndData;
use crate::Model;
use alloc::vec;
use std::collections::HashMap;

#[test]
fn par_data_dim1_independent() {
    let m = Model::new();

    let i = m.set();
    let s = m.par_of(i);

    let ds = s.data(&(), |_, _| 42);
    assert_eq!(ds.value(&[3]), 42.0);

    let ds = s.data(&(), |_, _| 12f32);
    assert_eq!(ds.value(&[3]), 12.0);

    let ds = s.data(&(), |_, i| i);
    assert_eq!(ds.value(&[3]), 3.0);

    let ds = s.data(&(), |_, i| i * 2 + 1);
    assert_eq!(ds.value(&[3]), 7.0);
}

#[test]
fn par_data_dim1_from_data() {
    let m = Model::new();

    let i = m.set();
    let s = m.par_of(i);

    let data = 42;
    let ds = s.data(&data, |d, _| *d);
    assert_eq!(ds.value(&[3]), 42.0);

    let data = 42;
    let ds = s.data(&data, |d, i| d + i);
    assert_eq!(ds.value(&[3]), 45.0);

    let data = vec![3, 5, 2, 1, 7];
    let ds = s.data(&data, |d, i| d[i]);
    assert_eq!(ds.value(&[3]), 1.0);

    let data: HashMap<usize, i64> = [(0, 3), (7, 2), (1, 5)].into_iter().collect();
    let ds = s.data(&data, |d, i| *d.get(&i).unwrap());
    assert_eq!(ds.value(&[7]), 2.0);
}
