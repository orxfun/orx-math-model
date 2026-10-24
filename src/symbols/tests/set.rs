use crate::elements::ElementContainer;
use crate::symbols::{Elements, ParIntData, Set, SetFun, SetOfIdx};
use crate::Model;
use alloc::boxed::Box;
use alloc::vec;
use core::fmt::Debug;

fn assert_elements<'d>(e: &mut Elements<'d>, s: Set<'_, 'd>, expected: &Elements<'d>) {
    for _ in 0..3 {
        unsafe { s.el.get() }.fill(e);
        assert_eq!(e, expected);
    }
}

fn add_some_more_sets(m: &Model<'_>) {
    for _ in 0..3 {
        let _ = m.set();
    }
}

#[test]
fn set_of_idx_get_set() {
    let data_int = vec![1, 2, 3];

    let m = Model::new();
    add_some_more_sets(&m);

    let s = m.set();

    let expected = unsafe { s.el.get() }.create_elements();
    let mut e = unsafe { s.el.get() }.create_elements();
    assert_elements(&mut e, s, &expected);

    add_some_more_sets(&m);

    // int

    s.el.update(|x| *x = SetFun::Usize(Box::new(|e| e.fill(data_int.iter().copied()))));
}
