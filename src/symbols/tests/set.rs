use crate::data_structs::ElemCont;
use crate::symbols::{IndicesFun, Set};
use crate::Model;
use alloc::boxed::Box;
use alloc::vec;

fn assert_elements(e: &mut ElemCont<usize>, s: Set<'_, '_>, expected: &[usize]) {
    for _ in 0..3 {
        unsafe { s.el.get() }.fill(e);
        assert_eq!(e.as_slice(), expected);
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
    let mut e = ElemCont::default();

    add_some_more_sets(&m);

    let s = m.set();

    assert_elements(&mut e, s, &[]);

    add_some_more_sets(&m);

    s.el.update(|x| *x = IndicesFun::new(Box::new(|e| e.fill(data.iter().copied()))));
    assert_elements(&mut e, s, &[1, 2, 3]);

    s.el.update(|x| *x = IndicesFun::new(Box::new(|e| e.fill(0..data.len()))));
    assert_elements(&mut e, s, &[0, 1, 2]);

    s.el.update(|x| *x = IndicesFun::new(Box::new(|e| e.fill(vec![1, 3, 5]))));
    assert_elements(&mut e, s, &[1, 3, 5]);
}
