use orx_math_model::*;
use std::collections::{BTreeMap, HashMap};

#[test]
fn sets() {
    let model = Model::new();

    let i = model.set("i");
    assert_eq!(format!("{i:?}"), "Set { key: \"i\", definition: \"\" }");

    let j = model.set("j").definition("vehicles");
    assert_eq!(
        format!("{j:?}"),
        "Set { key: \"j\", definition: \"vehicles\" }"
    );

    let k = model.set("k").definition("vehicles").definition("tasks");
    assert_eq!(
        format!("{k:?}"),
        "Set { key: \"k\", definition: \"tasks\" }"
    );
}

#[test]
fn set_reference_equality() {
    // same model
    let m1 = Model::new();
    let i = m1.set("i");
    let i2 = i;
    let j = m1.set("i");
    assert_eq!(i, i2);
    assert_ne!(i, j);
    assert_ne!(i2, j);

    // different models
    let m2 = Model::new();
    let k = m2.set("k");
    assert_ne!(k, i);
    assert_ne!(k, i2);
    assert_ne!(k, j);
}

#[test]
fn set_ref_as_keys_of_hashmap() {
    let m = Model::new();

    let i = m.set("i").definition("ii");
    let j = m.set("j").definition("kkk");
    let i2 = i;

    let mut map = HashMap::new();
    map.insert(i, 0);
    assert_eq!(map.len(), 1);
    map.insert(j, 1);
    assert_eq!(map.len(), 2);

    let a = map.contains_key(&i);

    for x in map.keys() {
        dbg!(x);
    }

    assert!(map.contains_key(&i));
    assert!(map.contains_key(&j));
    assert!(map.contains_key(&i2));

    assert_eq!(map.get(&i), Some(&0));
    assert_eq!(map.get(&i2), Some(&0));
    assert_eq!(map.get(&j), Some(&1));
}

#[test]
fn set_ref_as_keys_of_btreemap() {
    let m = Model::new();

    let i = m.set("i");
    let j = m.set("j");
    let i2 = i;

    let mut map = BTreeMap::new();
    map.insert(i, 0);
    map.insert(j, 1);

    assert!(map.contains_key(&i));
    assert!(map.contains_key(&j));
    assert!(map.contains_key(&i2));

    assert_eq!(map.get(&i), Some(&0));
    assert_eq!(map.get(&i2), Some(&0));
    assert_eq!(map.get(&j), Some(&1));
}
