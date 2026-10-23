use crate::symbols::{Elements, Set, SetData};
use crate::Model;
use alloc::vec;

fn assert_elements(e: &mut Elements, p: &Set<'_, '_>, expected: &[usize]) {
    for _ in 0..3 {
        p.value().fill(e);
        assert_eq!(e.as_slice(), expected);
    }
}

fn add_some_more_sets(m: &Model<'_>) {
    for _ in 0..3 {
        let _ = m.set();
    }
}

#[test]
fn set_get_set() {
    let data = vec![1, 2, 3];
    let m = Model::new();
    let mut elements = Elements::default();

    add_some_more_sets(&m);

    let s = m.set();

    s.value().fill(&mut elements);
    assert_elements(&mut elements, &s, &[]);

    add_some_more_sets(&m);

    s.set(SetData::new(|| data.iter().copied()));
    assert_elements(&mut elements, &s, &[1, 2, 3]);

    s.set(SetData::new(|| 0..data.len()));
    assert_elements(&mut elements, &s, &[0, 1, 2]);

    s.set(SetData::new(|| vec![1, 3, 5]));
    assert_elements(&mut elements, &s, &[1, 3, 5]);
}
