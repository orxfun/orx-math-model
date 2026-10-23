use crate::symbols::{Elements, Set, SetData};
use crate::Model;
use alloc::vec;

fn assert_123(e: &mut Elements, p: &Set<'_, '_>) {
    for _ in 0..3 {
        p.value().fill(e);
        assert_eq!(e.as_slice(), &vec![1, 2, 3]);
    }
}

fn add_some_more_sets(m: &Model<'_>) {
    for _ in 0..3 {
        let _ = m.set();
    }
}

#[test]
fn set_get_set() {
    let m = Model::new();
    let mut elements = Elements::default();

    add_some_more_sets(&m);

    let s = m.set();

    s.value().fill(&mut elements);
    assert_eq!(elements.as_slice(), &vec![]);

    add_some_more_sets(&m);

    let data = vec![1, 2, 3];
    s.set(SetData::new(|| data.iter().copied()));
}
