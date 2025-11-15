use super::knapsack_model::*;
use crate::draft::set::*;
use crate::draft::set_data::*;
use crate::no_std_types::Map;

pub struct ItemsMap(Map<&'static str, Item>);

impl SetData0 for ItemsMap {
    type Set = Items;

    fn keys(&self) -> impl Iterator<Item = <Self::Set as Set>::Key> + '_ {
        self.0.keys().copied()
    }

    fn element(&self, key: <Self::Set as Set>::Key) -> &<Self::Set as Set>::Elem {
        &self.0[&key]
    }
}
