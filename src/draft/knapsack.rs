use crate::draft::par::*;
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

struct Weight;
impl Par for Weight {
    type Value = u64;
}
impl Par1 for Weight {
    type S0 = Items;
    fn value(&self, element: &<Self::S0 as Set>::Elem) -> Self::Value {
        element.weight
    }
}

struct Value;
impl Par for Value {
    type Value = f32;
}
impl Par1 for Value {
    type S0 = Items;
    fn value(&self, element: &<Self::S0 as Set>::Elem) -> Self::Value {
        element.value
    }
}

fn model() {
    let items = Items;
    let weight = Weight;
    let value = Value;
}
