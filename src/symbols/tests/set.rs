use crate::symbols::{Elements, Set, SetFun};
use crate::Model;
use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec;

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
fn set_get_set() {
    let data_int = vec![1, 2, 3];
    let data_str = vec![1.to_string(), 2.to_string(), 3.to_string()];

    let m = Model::new();
    add_some_more_sets(&m);

    let s = m.set();

    let expected = unsafe { s.el.get() }.create_elements();
    let mut e = unsafe { s.el.get() }.create_elements();
    assert_elements(&mut e, s, &expected);

    add_some_more_sets(&m);

    // int

    s.el.update(|x| *x = SetFun::Usize(Box::new(|e| e.fill(data_int.iter().copied()))));
    let mut e = unsafe { s.el.get() }.create_elements();
    assert_elements(&mut e, s, &Elements::Usize(vec![1, 2, 3].into()));

    s.el.update(|x| *x = SetFun::Usize(Box::new(|e| e.fill(0..data_int.len()))));
    assert_elements(&mut e, s, &Elements::Usize(vec![0, 1, 2].into()));

    s.el.update(|x| *x = SetFun::Usize(Box::new(|e| e.fill(vec![1, 3, 5]))));
    assert_elements(&mut e, s, &Elements::Usize(vec![1, 3, 5].into()));

    // str

    s.el.update(|x| *x = SetFun::Str(Box::new(|e| e.fill(data_str.iter().map(|x| x.as_str())))));
    let mut e = unsafe { s.el.get() }.create_elements();
    assert_elements(&mut e, s, &Elements::Str(vec!["1", "2", "3"].into()));

    s.el.update(|x| *x = SetFun::Str(Box::new(|e| e.fill(vec!["1", "3", "5"]))));
    assert_elements(&mut e, s, &Elements::Str(vec!["1", "3", "5"].into()));
}
