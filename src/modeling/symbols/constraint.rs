use crate::modeling::stages::ModelStages;
use crate::modeling::symbol_collections::symbol_collection::SymbolCollection;
use crate::modeling::{
    reference::{HasRef, SymRef},
    symbols::data::forall::ForAllAndCoreRef,
};
use std::fmt::{Debug, Display};

use super::data::constraint_data::ConstraintData;

#[derive(Clone, Copy)]
pub struct Constraint<B: ModelStages>(SymRef<B>);
impl<B: ModelStages> HasRef<B> for Constraint<B> {
    fn new(index: SymRef<B>) -> Self {
        Self(index)
    }
    fn sym_ref(&self) -> &SymRef<B> {
        &self.0
    }
}

impl<B: ModelStages> Display for Constraint<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let expression = &self.data().constraint_expression;
        let forall = ForAllAndCoreRef(&self.data().forall, self.sym_ref().core_ref.clone());

        write!(f, "{} | {}", expression, forall)
    }
}

impl<B: ModelStages> Debug for Constraint<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let expression = &self.data().constraint_expression;
        let forall = ForAllAndCoreRef(&self.data().forall, self.sym_ref().core_ref.clone());

        f.debug_struct("Constraint")
            .field("str", &self.to_string())
            .field("expression", &expression)
            .field("forall", &forall)
            .finish()
    }
}

// data
impl<B: ModelStages> Constraint<B> {
    pub(crate) fn data(&self) -> &ConstraintData<B> {
        self.core().constraints.get(self.sym_idx())
    }
}
