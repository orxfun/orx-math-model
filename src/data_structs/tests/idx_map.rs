use crate::data_structs::IdxMap;
use alloc::string::ToString;
use alloc::{vec, vec::Vec};

#[test]
fn idx_map_string() {
    let data = vec!["a".to_string(), "b".to_string()];
    let map = IdxMap::from(data);

    assert_eq!(map.val_at(0), Some(&"a".to_string()));
    assert_eq!(map.val_at(1), Some(&"b".to_string()));
    assert_eq!(map.val_at(2), None);

    assert_eq!(map.pos_of(&"a".to_string()), Some(0));
    assert_eq!(map.pos_of(&"b".to_string()), Some(1));
    assert_eq!(map.pos_of(&"c".to_string()), None);
}

#[test]
fn idx_map_str() {
    let owned_data = vec!["a".to_string(), "b".to_string()];
    let data: Vec<_> = owned_data.iter().map(|x| x.as_str()).collect();
    let map = IdxMap::from(data);

    assert_eq!(map.val_at(0), Some(&"a"));
    assert_eq!(map.val_at(1), Some(&"b"));
    assert_eq!(map.val_at(2), None);

    assert_eq!(map.pos_of(&"a"), Some(0));
    assert_eq!(map.pos_of(&"b"), Some(1));
    assert_eq!(map.pos_of(&"c"), None);
}
