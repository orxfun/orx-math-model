use crate::no_std_types::Map;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ops::Range;

pub trait Set0 {
    type Key;
    type Elem;
}

pub trait SetData0 {
    type Set: Set0;
    fn keys(&self) -> Box<dyn Iterator<Item = <Self::Set as Set0>::Key> + '_>;
}

// // set

// pub trait Set0 {
//     type Key;

//     type Element;

//     fn keys(&self) -> Box<dyn Iterator<Item = Self::Key> + '_>;
// }

// pub trait SetMap0: Set0 {
//     fn element(&self, key: Self::Key) -> &Self::Element;
// }

// impl Set0 for Range<usize> {
//     type Key = usize;

//     type Element = ();

//     fn keys(&self) -> Box<dyn Iterator<Item = Self::Key> + '_> {
//         Box::new(self.clone().into_iter())
//     }
// }

// impl<E> Set0 for Vec<E> {
//     type Key = usize;

//     type Element = E;

//     fn keys(&self) -> Box<dyn Iterator<Item = Self::Key> + '_> {
//         Box::new(0..self.len())
//     }
// }
// impl<E> SetMap0 for Vec<E> {
//     fn element(&self, key: Self::Key) -> &Self::Element {
//         &self[key]
//     }
// }

// impl<E> Set0 for Map<usize, E> {
//     type Key = usize;

//     type Element = E;

//     fn keys(&self) -> Box<dyn Iterator<Item = Self::Key> + '_> {
//         Box::new(self.keys().copied())
//     }
// }
// impl<E> SetMap0 for Map<usize, E> {
//     fn element(&self, key: Self::Key) -> &Self::Element {
//         &self[&key]
//     }
// }

// // par

// pub trait Par1 {
//     type Value;

//     type Set: SetMap0;

//     fn get(element: &<Self::Set as Set0>::Element) -> Self::Value;

//     fn value(set: &Self::Set, key: <Self::Set as Set0>::Key) -> Self::Value {
//         let element = set.element(key);
//         Self::get(element)
//     }
// }

// // knapsack

// struct Item {
//     cost: u64,
//     weight: f64,
// }

// struct ItemCost;
// impl Par1 for ItemCost {
//     type Value = u64;
//     type Set = Vec<Item>;
//     fn get(element: &<Self::Set as Set0>::Element) -> Self::Value {
//         element.cost
//     }
// }

// fn abc() {
//     fn take_set(_: impl Set0) {}

//     let v: Vec<Item> = Default::default();
//     take_set(v);
// }
