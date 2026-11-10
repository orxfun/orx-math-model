use crate::index_types::IdxInVariant;
use crate::{elements::Elements, Model};
use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};

#[test]
fn idx_cat_int() {
    let values = vec![1usize, 2, 7, 9];

    let m = Model::new();
    let set = m.set_cat_int();
    let idx = set.idx();

    let mut elements: Elements<usize> = vec![].into();
    unsafe { set.el.get_mut() }.bind_from(|| values.iter().copied());

    let iter = unsafe { idx.el.get() }.iter(&mut elements);
    for (i, _) in iter.enumerate() {
        let val = unsafe { idx.el.get() }.value_as_variant();
        assert_eq!(val, IdxInVariant::Usize(values[i]));
    }
}

#[test]
fn idx_cat_str() {
    let values: Vec<_> = [1, 2, 7, 9].map(|x| x.to_string()).into_iter().collect();

    let m = Model::new();
    let set = m.set_cat_str();
    let idx = set.idx();

    let mut elements: Elements<&String> = vec![].into();
    unsafe { set.el.get_mut() }.bind_from(|| values.iter());

    let iter = unsafe { idx.el.get() }.iter(&mut elements);
    for (i, _) in iter.enumerate() {
        let val = unsafe { idx.el.get() }.value_as_variant();
        assert_eq!(val, IdxInVariant::Str(&values[i]));
    }
}

#[test]
fn idx_ord() {
    let values = vec![1i64, 2, 7, 9];

    let m = Model::new();
    let set = m.set_ord();
    let idx = set.idx();

    let mut elements: Elements<i64> = vec![].into();
    unsafe { set.el.get_mut() }.bind_from(|| values.iter().copied());

    let iter = unsafe { idx.el.get() }.iter(&mut elements);
    for (i, _) in iter.enumerate() {
        let val = unsafe { idx.el.get() }.value_as_variant();
        assert_eq!(val, IdxInVariant::I64(values[i]));
    }
}
