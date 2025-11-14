use crate::data::par_data::par_and_data::ParDataCore;
use crate::Model;
use alloc::vec;
use alloc::vec::Vec;

#[test]
fn par_data_dim0_independent() {
    let m = Model::new();

    let s = m.par();

    let ds = s.data(&(), |_| 42);
    assert_eq!(ds.value(&[]), 42.0);

    let ds = s.data(&(), |_| 12f32);
    assert_eq!(ds.value(&[]), 12.0);
}

#[test]
fn par_data_dim0_from_data() {
    let m = Model::new();

    let s = m.par();

    let data = 42;
    let ds = s.data(&data, |d| *d);
    assert_eq!(ds.value(&[]), 42.0);

    struct Data {
        things: Vec<usize>,
    }
    let data = Data {
        things: vec![0, 1, 2, 3],
    };
    let ds = s.data(&data, |d| d.things.len());
    assert_eq!(ds.value(&[]), 4.0);
}
