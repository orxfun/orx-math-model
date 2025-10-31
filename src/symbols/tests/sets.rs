use crate::{symbols::symbol_map::SymbolMap, *};
use alloc::format;
use alloc::vec;

#[test]
fn sets() {
    let model = Model::new();

    let i = model.set(vec![]).key("i");
    assert_eq!(format!("{i:?}"), "Set { key: \"i\", definition: \"\" }");

    let j = model.set(vec![]).key("j").definition("vehicles");
    assert_eq!(
        format!("{j:?}"),
        "Set { key: \"j\", definition: \"vehicles\" }"
    );

    let k = model
        .set(vec![])
        .key("k")
        .definition("vehicles")
        .definition("tasks");
    assert_eq!(
        format!("{k:?}"),
        "Set { key: \"k\", definition: \"tasks\" }"
    );
}

#[test]
fn set_reference_equality() {
    // same model
    let m1 = Model::new();
    let i = m1.set(vec![]).key("i");
    let i2 = i;
    let j = m1.set(vec![]).key("i");
    assert_eq!(i, i2);
    assert_ne!(i, j);
    assert_ne!(i2, j);

    // different models
    let m2 = Model::new();
    let k = m2.set(vec![]).key("k");
    assert_ne!(k, i);
    assert_ne!(k, i2);
    assert_ne!(k, j);
}

#[test]
fn set_in_symbol_map() {
    let m = Model::new();

    let i = m.set(vec![]).key("i");
    let j = m.set(vec![]).key("j");
    let i2 = i;

    let m2 = Model::new();
    let k = m2.set(vec![]).key("i");

    let mut map = SymbolMap::new();
    map.insert(i, 0);
    assert_eq!(map.len(), 1);
    map.insert(j, 1);
    assert_eq!(map.len(), 2);

    assert!(map.contains_key(i));
    assert!(map.contains_key(j));
    assert!(map.contains_key(i2));
    assert!(!map.contains_key(k));

    assert_eq!(map.get(i), Some(&0));
    assert_eq!(map.get(i2), Some(&0));
    assert_eq!(map.get(j), Some(&1));
    assert_eq!(map.get(k), None);
}
