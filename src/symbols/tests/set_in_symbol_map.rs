use crate::{symbols::symbol_map::SymbolMap, Model};

#[test]
fn set_in_symbol_map() {
    let m = Model::new();

    let i = m.set();
    let j = m.set();
    let i2 = i;

    let m2 = Model::new();
    let k = m2.set();

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
