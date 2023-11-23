use crate::modeling::stages::ModelStages;
use crate::modeling::symbol_collections::symbol_collection::SymbolCollection;
use crate::modeling::{
    ops::sum::core::get_sumoversets_and_expr,
    reference::{HasRef, SymRef},
};
use std::fmt::{Debug, Display};

use super::data::sum_data::SumData;

#[derive(Clone, Copy)]
pub struct Sum<B: ModelStages>(SymRef<B>);
impl<B: ModelStages> HasRef<B> for Sum<B> {
    fn new(index: SymRef<B>) -> Self {
        Self(index)
    }
    fn sym_ref(&self) -> &SymRef<B> {
        &self.0
    }
}

impl<B: ModelStages> Display for Sum<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (sum_over_set_indices, expression) = get_sumoversets_and_expr(self.clone());
        let sets = &expression.core().sets;
        let sets: Vec<_> = sum_over_set_indices
            .iter()
            .map(|symref| sets.get(symref.sym_idx).key.to_string())
            .collect();
        let set_keys = sets.join(",");

        write!(f, "sum({} | {})", set_keys, expression)
    }
}

impl<B: ModelStages> Debug for Sum<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (sum_over_set_indices, expression) = get_sumoversets_and_expr(self.clone());
        let sets = &expression.core().sets;
        let sets: Vec<_> = sum_over_set_indices
            .iter()
            .map(|symref| sets.get(symref.sym_idx).key.to_string())
            .collect();

        f.debug_struct("Sum")
            .field("str", &self.to_string())
            .field("num_sum_over_sets", &sets.len())
            .field("sum_over_sets", &sets)
            .field("expression", &expression)
            .finish()
    }
}

// data
impl<B: ModelStages> Sum<B> {
    pub(crate) fn data(&self) -> &SumData<B> {
        self.core().sums.get(self.sym_idx())
    }
}
