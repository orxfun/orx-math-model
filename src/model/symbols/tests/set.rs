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

    p.set(Set0Gen::new(|| vec![1, 2, 3]));
    p.value().fill(&mut elements);
    assert_eq!(elements.as_slice(), &vec![1, 2, 3]);

    p.set(Set0Gen::new(|| 1..4));
    p.value().fill(&mut elements);
    assert_eq!(elements.as_slice(), &vec![1, 2, 3]);

    let data = Rc::new(vec![1, 2, 3]);
    let d = data.clone();
    p.set(Set0Gen::new2(Box::new(move |e| {
        e.fill_with(d.iter().copied())
    })));
}
