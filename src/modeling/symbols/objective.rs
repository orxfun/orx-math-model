use crate::modeling::reference::{HasRef, SymRef};
use crate::modeling::stages::ModelStages;
use crate::modeling::symbol_collections::symbol_collection::SymbolCollection;
use std::fmt::{Debug, Display};

use super::data::objective_data::ObjectiveData;

#[derive(Clone, Copy)]
pub struct Objective<B: ModelStages>(SymRef<B>);
impl<B: ModelStages> HasRef<B> for Objective<B> {
    fn new(index: SymRef<B>) -> Self {
        Self(index)
    }
    fn sym_ref(&self) -> &SymRef<B> {
        &self.0
    }
}

impl<B: ModelStages> Display for Objective<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let core = self.core();
        let data = core.objectives.get(self.sym_idx());
        let expression = core
            .expressions
            .get(data.expression_ref.sym_idx)
            .symbol
            .clone();
        write!(f, "{} {}", data.direction, expression)
    }
}

impl<B: ModelStages> Debug for Objective<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let core = self.core();
        let data = core.objectives.get(self.sym_idx());
        let expression = core
            .expressions
            .get(data.expression_ref.sym_idx)
            .symbol
            .clone();

        f.debug_struct("Objective")
            .field("str", &self.to_string())
            .field("direction", &data.direction)
            .field("expression", &expression)
            .finish()
    }
}

// data
impl<B: ModelStages> Objective<B> {
    pub(crate) fn data(&self) -> &ObjectiveData<B> {
        self.core().objectives.get(self.sym_idx())
    }
}
