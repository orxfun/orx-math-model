use crate::{model::Set0Gen, Model};
use alloc::{rc::Rc, vec, vec::Vec};

#[test]
fn read_write_set_d0() {
    let m = Model::new();

    for _ in 0..3 {
        let _ = m.set();
    }

    let p = m.set();
    assert_eq!(p.value().generate().collect::<Vec<_>>(), vec![]);

    for _ in 0..3 {
        let _ = m.set();
    }

    p.set(Set0Gen::new(|| vec![1, 2, 3]));
    assert_eq!(p.value().generate().collect::<Vec<_>>(), vec![1, 2, 3]);

    p.set(Set0Gen::new(|| 1..4));
    assert_eq!(p.value().generate().collect::<Vec<_>>(), vec![1, 2, 3]);

    let data = vec![1, 2, 3];
    let data = Rc::new(data);
    let data2 = data.clone();
    p.set(Set0Gen::new(move || data2.iter().copied()));
    assert_eq!(p.value().generate().collect::<Vec<_>>(), vec![1, 2, 3]);
}
