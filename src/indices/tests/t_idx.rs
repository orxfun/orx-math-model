use crate::{elements::Elements, Model};
use alloc::string::ToString;
use alloc::{vec, vec::Vec};

#[test]
fn idx_cat_int() {
    let values = vec![1, 2, 7, 9];
    let mut elements = Elements::from(vec![]);

    let m = Model::new();
    let set = m.set_cat();
    let idx = set.idx();

    let mut iter = idx.iter(&mut elements);
    assert!(iter.next().is_none());

    unsafe { set.el.get_mut() }
        .as_cat_int_mut()
        .bind(|| values.iter().copied());
    let iter = idx.iter(&mut elements);
    for (i, _) in iter.enumerate() {
        let val = unsafe { idx.el.get() }.val_ptr_int().val();
        assert_eq!(val, values[i]);
    }
}

#[test]
fn idx_cat_str() {
    let values: Vec<_> = [1, 2, 7, 9].map(|x| x.to_string()).into_iter().collect();
    let mut elements = Elements::from(vec![]);

    let m = Model::new();
    let set = m.set_cat_str();
    let idx = set.idx();

    let mut iter = idx.iter(&mut elements);
    assert!(iter.next().is_none());

    unsafe { set.el.get_mut() }
        .as_cat_str_mut()
        .bind(|| values.iter().map(|x| x.as_str()));
    let iter = idx.iter(&mut elements);
    for (i, _) in iter.enumerate() {
        let val = unsafe { idx.el.get() }.val_ptr_str().val();
        assert_eq!(val, values[i].as_str());
    }
}

#[test]
fn idx_ord() {
    let values = vec![1, 2, 7, 9];
    let mut elements = Elements::from(vec![]);

    let m = Model::new();
    let set = m.set_ord();
    let idx = set.idx();

    let mut iter = idx.iter(&mut elements);
    assert!(iter.next().is_none());

    unsafe { set.el.get_mut() }
        .as_ord_mut()
        .bind(|| values.iter().copied());
    let iter = idx.iter(&mut elements);
    for (i, _) in iter.enumerate() {
        let val = unsafe { idx.el.get() }.val_ptr_int().val();
        assert_eq!(val, values[i]);
    }
}
