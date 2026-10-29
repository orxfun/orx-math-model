use crate::symbols::ElStr;
use crate::Model;

fn assert_idx_value<'d>(idx_symbols: &[ElStr<'_, 'd>], expected: &'d str) {
    for i in idx_symbols {
        assert_eq!(unsafe { i.el.get() }.value(), expected);
    }
}

#[test]
fn idx_get_set() {
    let m = Model::new();

    let s = m.set_str();
    let idx = s.idx();
    let i = idx;

    assert_idx_value(&[idx, i], "");

    i.el.update(|x| x.set_value("7"));
    assert_idx_value(&[idx, i], "7");
}
