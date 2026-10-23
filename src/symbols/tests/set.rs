use crate::symbols::{Elements, Set, SetData};
use crate::Model;
use alloc::vec;

fn assert_elements(e: &mut Elements, set_symbols: &[Set<'_, '_>], expected: &[usize]) {
    for s in set_symbols {
        for _ in 0..3 {
            unsafe { s.el.get() }.fill(e);
            assert_eq!(e.as_slice(), expected);
        }
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

    let set = m.set();
    let s = set;

    assert_elements(&mut elements, &[s, set], &[]);

    add_some_more_sets(&m);

    s.el.set(SetData::new(|| data.iter().copied()));
    assert_elements(&mut elements, &[s, set], &[1, 2, 3]);

    s.el.set(SetData::new(|| 0..data.len()));
    assert_elements(&mut elements, &[s, set], &[0, 1, 2]);

    s.el.set(SetData::new(|| vec![1, 3, 5]));
    assert_elements(&mut elements, &[s, set], &[1, 3, 5]);
}
