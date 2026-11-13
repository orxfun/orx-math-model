use crate::{Model, ValInt, ValNum};
use alloc::vec;

#[test]
fn val_int() {
    type V = ValInt;

    // data
    let values = vec![1i64, 2, 7, 9];

    // modeling
    let m = Model::default();

    // modeling & binding
    let val = m.val::<V>(|| values[2]);

    // running
    assert_eq!(unsafe { val.el.get() }.eval::<V>(), values[2]);
}

#[test]
fn val_num() {
    type V = ValNum;

    // data
    let values = vec![1.0, 2.0, 7.0, 9.0];

    // modeling
    let m = Model::default();

    // modeling & binding
    let val = m.val::<V>(|| values[2]);

    // running
    assert_eq!(unsafe { val.el.get() }.eval::<V>(), values[2]);
}
