use crate::draft::par::*;
use crate::draft::set::*;

struct Item {
    name: &'static str,
    weight: u64,
    value: f32,
}

#[derive(Clone, Copy)]
struct Items;
impl Set for Items {
    type Key = &'static str;
    type Elem = Item;
}
impl Set0 for Items {}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
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
    let i = Items;
    let weight = Weight;
    let value = Value;
}
