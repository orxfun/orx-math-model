use crate::elements::ElementsOfIdx;
use crate::symbols::{ParData, SetOfIdx};
use crate::Model;
use alloc::vec;

fn assert_elements(e: &mut ElementsOfIdx, set_symbols: &[SetOfIdx<'_, '_>], expected: &[usize]) {
    for s in set_symbols {
        for _ in 0..3 {
            unsafe { s.el.get() }.fill(e);
            assert_eq!(e.as_slice(), expected);
        }
    }
}

fn add_some_more_sets(m: &Model<'_>) {
    for _ in 0..3 {
        let _ = m.set_of_idx();
    }
}

#[test]
fn set_of_idx_get_set() {
    let data = vec![1, 2, 3];

    let m = Model::new();
    let mut elements = ElementsOfIdx::default();

    add_some_more_sets(&m);

    let set = m.set_of_idx();
    let s = set;

    assert_elements(&mut elements, &[s, set], &[]);

    add_some_more_sets(&m);

    s.el.update(|x| x.update_fun_from_gen(|| data.iter().copied()));
    assert_elements(&mut elements, &[s, set], &[1, 2, 3]);

    s.el.update(|x| x.update_fun_from_gen(|| 0..data.len()));
    assert_elements(&mut elements, &[s, set], &[0, 1, 2]);

    s.el.update(|x| x.update_fun_from_gen(|| vec![1, 3, 5]));
    assert_elements(&mut elements, &[s, set], &[1, 3, 5]);
}

// range

#[test]
fn range_get_set() {
    let m = Model::new();

    let p = m.par();
    p.el.set(ParData::new(2.0));

    let q = m.par();
    q.el.set(ParData::new(6.0));
}
