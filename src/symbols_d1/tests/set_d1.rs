use crate::symbols::{Elements, Set, SetData};
use crate::symbols_d1::{SetD1, SetD1Data};
use crate::Model;
use alloc::vec;

fn assert_elements(e: &mut Elements, set_symbols: &[SetD1<'_, '_>], i0: usize, expected: &[usize]) {
    for s in set_symbols {
        for _ in 0..3 {
            unsafe { s.el.get() }.fill(i0, e);
            assert_eq!(e.as_slice(), expected);
        }
    }
}

fn add_some_more_sets(m: &Model<'_>) {
    for _ in 0..3 {
        let x = m.set();
        let _ = m.set_d1(x);
    }
}

#[test]
fn set_get_set_d1() {
    let data = vec![vec![0], vec![1, 2, 3], vec![4, 5], vec![6, 7, 8, 9]];

    let m = Model::new();
    let mut elements = Elements::default();

    add_some_more_sets(&m);

    let s0 = m.set();
    let set = m.set_d1(s0);
    let s = set;

    assert_elements(&mut elements, &[s, set], 0, &[]);

    add_some_more_sets(&m);

    s.el.set(SetD1Data::new(s0, |i| 0..i));
    assert_elements(&mut elements, &[s, set], 0, &[]);
    assert_elements(&mut elements, &[s, set], 1, &[0]);
    assert_elements(&mut elements, &[s, set], 3, &[0, 1, 2]);

    s.el.set(SetD1Data::new(s0, |i| data[i].iter().copied()));
    assert_elements(&mut elements, &[s, set], 0, &[0]);
    assert_elements(&mut elements, &[s, set], 1, &[1, 2, 3]);
    assert_elements(&mut elements, &[s, set], 2, &[4, 5]);
    assert_elements(&mut elements, &[s, set], 3, &[6, 7, 8, 9]);
}
