use crate::{set_of, Model, SymbolRef};
use alloc::format;
use alloc::vec;

#[test]
fn set_with_values_d0() {
    let m = Model::new();

    let i = m.set().values(vec![1, 2, 3]);

    let j = m.set().values(4..8);
}
