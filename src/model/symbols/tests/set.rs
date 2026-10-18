use crate::model::symbols::{Set, Set2};
use crate::model::{Elements, Set0Gen, Set0Gen2};
use crate::Model;
use alloc::boxed::Box;
use alloc::{rc::Rc, vec};

fn assert(e: &mut Elements, p: &Set<'_>) {
    for _ in 0..3 {
        p.value().fill(e);
        assert_eq!(e.as_slice(), &vec![1, 2, 3]);
    }
}

fn assert2(e: &mut Elements, p: &Set2<'_, '_>) {
    for _ in 0..3 {
        p.value().fill(e);
        assert_eq!(e.as_slice(), &vec![1, 2, 3]);
    }
}

#[test]
fn abc() {
    let m = Model::new();
    let mut elements = Elements::default();

    for _ in 0..3 {
        let _ = m.set();
    }

    let p = m.set2();
    p.value().fill(&mut elements);
    assert_eq!(elements.as_slice(), &vec![]);

    for _ in 0..3 {
        let _ = m.set();
    }
    p.set(Set0Gen2::new(Box::new(|e| e.fill_with(vec![1, 2, 3]))));
    assert2(&mut elements, &p);

    let data = vec![1, 2, 3];
    p.set(Set0Gen2::new(Box::new(|e| {
        e.fill_with(data.iter().copied())
    })));
    let _ = &data;
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
    p.set(Set0Gen::new(Box::new(|e| e.fill_with(vec![1, 2, 3]))));
    assert(&mut elements, &p);

    p.set(Set0Gen::new(Box::new(|e| e.fill_with(1..4))));
    assert(&mut elements, &p);

    let data = Rc::new(vec![1, 2, 3]);
    let d = data.clone();
    p.set(Set0Gen::new(Box::new(move |e| {
        e.fill_with(d.iter().copied())
    })));
    assert(&mut elements, &p);
}
