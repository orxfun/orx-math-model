use crate::symbols::{Elements, Set};
use crate::symbols_d1::SetD1;
use crate::Model;
use alloc::vec;

fn add_some_more_sets(m: &Model<'_>) {
    for _ in 0..3 {
        let x = m.set();
        let _ = m.set_d1(x);
    }
}

#[test]
fn set_get_set_d1() {
    fn assert_elements(
        e: &mut Elements,
        set_symbols: &[SetD1<'_, '_>],
        i0: usize,
        expected: &[usize],
    ) {
        for s in set_symbols {
            for _ in 0..3 {
                unsafe { s.el.get() }.fill(i0, e);
                assert_eq!(e.as_slice(), expected);
            }
        }
    }

    let data = vec![vec![0], vec![1, 2, 3], vec![4, 5], vec![6, 7, 8, 9]];

    let m = Model::new();
    let mut elements = Elements::default();

    add_some_more_sets(&m);

    let s0 = m.set();
    let set = m.set_d1(s0);
    let s = set;

    assert_elements(&mut elements, &[s, set], 0, &[]);

    add_some_more_sets(&m);

    s.el.update(|x| x.update_fun_from_gen(|i| 0..i));
    assert_elements(&mut elements, &[s, set], 0, &[]);
    assert_elements(&mut elements, &[s, set], 1, &[0]);
    assert_elements(&mut elements, &[s, set], 3, &[0, 1, 2]);

    s.el.update(|x| x.update_fun_from_gen(|i| data[i].iter().copied()));
    assert_elements(&mut elements, &[s, set], 0, &[0]);
    assert_elements(&mut elements, &[s, set], 1, &[1, 2, 3]);
    assert_elements(&mut elements, &[s, set], 2, &[4, 5]);
    assert_elements(&mut elements, &[s, set], 3, &[6, 7, 8, 9]);

    let idx = s0.idx();
    for (i, expected) in data.iter().enumerate() {
        idx.el.update(|x| x.set_value(i));
        let generated = set.set(idx);
        unsafe { generated.el.get() }.fill(&mut elements);
        assert_eq!(elements.as_slice(), expected);
    }
}

#[test]
fn set_from_set_d1() {
    fn assert_elements(e: &mut Elements, set_symbols: &[Set<'_, '_>], expected: &[usize]) {
        for s in set_symbols {
            for _ in 0..1 {
                unsafe { s.el.get() }.fill(e);
                assert_eq!(e.as_slice(), expected);
            }
        }
    }

    let data = vec![vec![0], vec![1, 2, 3], vec![4, 5], vec![6, 7, 8, 9]];

    let m = Model::new();
    let mut elements = Elements::default();

    add_some_more_sets(&m);

    let s0 = m.set();
    let s1 = m.set_d1(s0);

    let i0 = s0.idx();

    let set = s1.set(i0);

    // before wire up set-d1 data

    assert_elements(&mut elements, &[set], &[]);

    i0.el.update(|x| x.set_value(1));
    assert_elements(&mut elements, &[set], &[]);

    // after wire up set-d1 data

    s1.el
        .update(|x| x.update_fun_from_gen(|i| data[i].iter().copied()));

    i0.el.update(|x| x.set_value(0));
    assert_elements(&mut elements, &[set], &[0]);

    i0.el.update(|x| x.set_value(1));
    assert_elements(&mut elements, &[set], &[1, 2, 3]);

    i0.el.update(|x| x.set_value(2));
    assert_elements(&mut elements, &[set], &[4, 5]);

    i0.el.update(|x| x.set_value(3));
    assert_elements(&mut elements, &[set], &[6, 7, 8, 9]);
}

#[test]
#[should_panic]
fn set_from_set_d1_wrong_idx() {
    let m = Model::new();
    let s0 = m.set();
    let s1 = m.set_d1(s0);

    let wrong_set = m.set();
    let wrong_idx = wrong_set.idx();

    let _ = s1.set(wrong_idx);
}
