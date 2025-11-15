use crate::{model::Model, sets::Element};
use std::string::ToString;

struct Item {
    name: &'static str,
    cost: f64,
    weight: u32,
}
impl Element for Item {
    type Idx = &'static str;
    fn idx(&self) -> Self::Idx {
        self.name
    }
}

#[test]
fn knapsack_sets() {
    let model = Model::default();

    let items = model.set1::<Item>().key("I");
    assert_eq!(items.to_string(), "I".to_string());
}
