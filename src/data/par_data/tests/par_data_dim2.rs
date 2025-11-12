use crate::data::par_data::par_and_data::ParAndData;
use crate::Model;
use alloc::vec;
use std::collections::HashMap;

#[test]
fn par_data_dim2_independent() {
    let m = Model::new();

    let i = m.set();
    let j = m.set_of(i);
    let s = m.par_of((i, j));

    let ds = s.data(&(), |_, _, _| 42);
    assert_eq!(ds.value(&[3, 1]), 42.0);

    let ds = s.data(&(), |_, _, _| 12f32);
    assert_eq!(ds.value(&[3, 1]), 12.0);

    let ds = s.data(&(), |_, i, j| i + j);
    assert_eq!(ds.value(&[3, 1]), 4.0);

    let ds = s.data(&(), |_, i, j| i * 2 + j);
    assert_eq!(ds.value(&[3, 1]), 7.0);
}

#[test]
fn par_data_dim2_from_data() {
    let m = Model::new();

    let i = m.set();
    let j = m.set_of(i);
    let s = m.par_of((i, j));

    let data = 42;
    let ds = s.data(&data, |d, _, _| *d);
    assert_eq!(ds.value(&[3, 1]), 42.0);

    let data = 42;
    let ds = s.data(&data, |d, i, j| *d + i - j);
    assert_eq!(ds.value(&[3, 1]), 44.0);

    let data = vec![vec![3, 2, 4, 6, 23, 1], vec![1, 3], vec![3, 3, 1, 2, 6]];
    let ds = s.data(&data, |d, i, j| d[i][j]);
    assert_eq!(ds.value(&[0, 3]), 6.0);

    let data: HashMap<(usize, usize), i64> = [((0, 3), 7), ((2, 1), 6), ((3, 3), 9)]
        .into_iter()
        .collect();
    let ds = s.data(&data, |d, i, j| *d.get(&(i, j)).unwrap());
    assert_eq!(ds.value(&[2, 1]), 6.0);
}
