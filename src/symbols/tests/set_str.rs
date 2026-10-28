use crate::data_structs::ElemCont;
use crate::symbols::{SetStr, SetStrFun};
use crate::Model;
use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec;

fn assert_elements<'d>(e: &mut ElemCont<&'d str>, s: SetStr<'_, 'd>, expected: &[&'d str]) {
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
fn set_get_set_str() {
    let data = vec![1.to_string(), 2.to_string(), 3.to_string()];

    let m = Model::new();
    let mut e = ElemCont::default();

    add_some_more_sets(&m);

    let s = m.set_str();

    assert_elements(&mut e, s, &[]);

    add_some_more_sets(&m);

    s.el.update(|x| *x = SetStrFun::new(Box::new(|e| e.fill(data.iter().map(|x| x.as_str())))));
    assert_elements(&mut e, s, &[&"1", "2", "3"]);

    s.el.update(|x| *x = SetStrFun::new(Box::new(|e| e.fill(vec!["1", "3", "5"]))));
    assert_elements(&mut e, s, &["1", "3", "5"]);
}
