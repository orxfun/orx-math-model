use crate::Model;
use crate::SymbolRef;

#[test]
fn set_reference_equality_different_model() {
    // same model
    let m1 = Model::new();
    let i = m1.set().key("i");
    let i2 = i;
    let j = m1.set().key("i");

    // different models
    let m2 = Model::new();
    let k = m2.set().key("k");
    assert_ne!(k, i);
    assert_ne!(k, i2);
    assert_ne!(k, j);
}

#[test]
fn set_reference_equality_same_model() {
    let m1 = Model::new();
    let i = m1.set().key("i");
    let i2 = i;
    let j = m1.set().key("i");
    assert_eq!(i, i2);
    assert_ne!(i, j);
    assert_ne!(i2, j);
}
