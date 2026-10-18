use crate::model::symbols::Set;
use crate::model::{Elements, Set0Gen};
use crate::Model;
use alloc::boxed::Box;
use alloc::{rc::Rc, vec};

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

    fn assert(e: &mut Elements, p: &Set<'_>) {
        for _ in 0..3 {
            p.value().fill(e);
            assert_eq!(e.as_slice(), &vec![1, 2, 3]);
        }
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
