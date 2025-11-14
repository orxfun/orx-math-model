use crate::draft::set::*;
struct Item {
    name: &'static str,
    weight: u64,
    value: f32,
}

struct Items;
impl Set for Items {
    type Key = &'static str;
    type Elem = Item;
}
impl Set0 for Items {}

fn model() {
    let items = Items;
}
