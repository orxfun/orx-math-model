use crate::elements::ElemCont;
use crate::symbols::{Indices, ParIntData};
use crate::Model;
use alloc::vec;

fn assert_elements(e: &mut ElemCont<usize>, set_symbols: &[Indices<'_, '_>], expected: &[usize]) {
    for s in set_symbols {
        for _ in 0..3 {
            unsafe { s.el.get() }.fill(e);
            assert_eq!(e.as_slice(), expected);
        }
    }
}

fn add_some_more_sets(m: &Model<'_>) {
    for _ in 0..3 {
        let _ = m.indices();
    }
}

// indices

#[test]
fn indices_get_set() {
    let data = vec![1, 2, 3];

    let m = Model::new();
    let mut elements = ElemCont::default();

    add_some_more_sets(&m);

    let set = m.indices();
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
    let mut elements = ElemCont::default();

    let p2 = m.par_int();
    p2.el.set(ParIntData::new(2));

    let p6 = m.par_int();
    p6.el.set(ParIntData::new(6));

    let s = m.range(2, 6);
    assert_elements(&mut elements, &[s], &[2, 3, 4, 5]);

    let s = m.range(p2, 6);
    assert_elements(&mut elements, &[s], &[2, 3, 4, 5]);

    let s = m.range(2, p6);
    assert_elements(&mut elements, &[s], &[2, 3, 4, 5]);

    let s = m.range(p2, p6);
    assert_elements(&mut elements, &[s], &[2, 3, 4, 5]);

    let s = m.range(p2 + 1, p6 + 2);
    assert_elements(&mut elements, &[s], &[3, 4, 5, 6, 7]);
}
