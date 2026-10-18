use crate::model::symbols::Set2;
use crate::model::{Elements, Set0Gen};
use crate::Model;
use alloc::vec;

fn assert(e: &mut Elements, p: &Set2<'_, '_>) {
    for _ in 0..3 {
        p.value().fill(e);
        assert_eq!(e.as_slice(), &vec![1, 2, 3]);
    }
}

#[test]
fn read_write_set_d0() {
    let m = Model::new();
    let mut elements = Elements::default();

    for _ in 0..3 {
        let _ = m.set();
    }

    let p = m.set();
    p.value().fill(&mut elements);
    assert_eq!(elements.as_slice(), &vec![]);

    for _ in 0..3 {
        let _ = m.set();
    }
    p.set(Set0Gen::new(|| vec![1, 2, 3]));
    assert(&mut elements, &p);

    let data = vec![1, 2, 3];
    p.set(Set0Gen::new(|| data.iter().copied()));
    assert(&mut elements, &p);
    let _ = &data;

    p.set(Set0Gen::new(|| 1..(1 + data.len())));
    assert(&mut elements, &p);
}
