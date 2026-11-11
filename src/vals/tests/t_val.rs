use crate::{vals::val_slots::ValValue, Model};
use alloc::vec;

#[test]
fn val_int() {
    // data
    let values = vec![1i64, 2, 7, 9];

    // modeling
    let m = Model::default();
    let val = m.val_int();

    // binding
    val.bind(|| values[2]).unwrap();

    // running
    assert_eq!(ValValue::Int(values[2]), unsafe { val.el.get() }.eval());
}

#[test]
fn val_num() {
    // data
    let values = vec![1.0, 2.0, 7.0, 9.0];

    // modeling
    let m = Model::default();
    let val = m.val_num();

    // binding
    val.bind(|| values[2]).unwrap();

    // running
    assert_eq!(ValValue::Num(values[2]), unsafe { val.el.get() }.eval());
}
