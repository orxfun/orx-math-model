use crate::elements::Elements;
use crate::symbols::Set;
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
fn set_d1_get_set() {
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
        let generated = set.at(idx);
        unsafe { generated.el.get() }.fill(&mut elements);
        assert_eq!(elements.as_slice(), expected);
    }
}

#[test]
fn set_from_set_d1() {
    fn assert_elements(e: &mut Elements, set: Set<'_, '_>, expected: &[usize]) {
        for _ in 0..1 {
            unsafe { set.el.get() }.fill(e);
            assert_eq!(e.as_slice(), expected);
        }
    }

    let data = vec![vec![0], vec![1, 2, 3], vec![4, 5], vec![6, 7, 8, 9]];

    let m = Model::new();
    let mut elements = Elements::default();

    add_some_more_sets(&m);

    let s0 = m.set();
    let s1 = m.set_d1(s0);

    let i0 = s0.idx();

    let s = s1.at(i0);

    // before wire up set-d1 data

    assert_elements(&mut elements, s, &[]);

    i0.el.update(|x| x.set_value(1));
    assert_elements(&mut elements, s, &[]);

    // after wire up set-d1 data

    s1.el
        .update(|x| x.update_fun_from_gen(|i| data[i].iter().copied()));

    i0.el.update(|x| x.set_value(0));
    assert_elements(&mut elements, s, &[0]);

    i0.el.update(|x| x.set_value(1));
    assert_elements(&mut elements, s, &[1, 2, 3]);

    i0.el.update(|x| x.set_value(2));
    assert_elements(&mut elements, s, &[4, 5]);

    i0.el.update(|x| x.set_value(3));
    assert_elements(&mut elements, s, &[6, 7, 8, 9]);

    // int expression as index

    let s = s1.at(i0 + 1);

    i0.el.update(|x| x.set_value(0));
    assert_elements(&mut elements, s, &[1, 2, 3]);

    i0.el.update(|x| x.set_value(1));
    assert_elements(&mut elements, s, &[4, 5]);

    i0.el.update(|x| x.set_value(2));
    assert_elements(&mut elements, s, &[6, 7, 8, 9]);

    // constant int as index

    let s = s1.at(2);
    assert_elements(&mut elements, s, &[4, 5]);
}
