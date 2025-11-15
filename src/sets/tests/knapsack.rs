use crate::model::Model;
use core::hash::Hash;
use std::string::ToString;

struct Item {
    id: usize,
    cost: f64,
    weight: u32,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Item {}
impl Hash for Item {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[test]
fn knapsack_sets() {
    let model = Model::default();

    let items = model.set1::<Item>().key("I");
    assert_eq!(items.to_string(), "I".to_string());
}
