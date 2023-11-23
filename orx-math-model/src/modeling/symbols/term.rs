use crate::modeling::reference::{HasRef, SymRef};
use crate::modeling::stages::ModelStages;
use crate::modeling::symbol_collections::symbol_collection::SymbolCollection;
use std::fmt::{Debug, Display};

use super::data::term_data::TermData;

#[derive(Clone, Copy)]
pub struct Term<B: ModelStages>(SymRef<B>);
impl<B: ModelStages> HasRef<B> for Term<B> {
    fn new(index: SymRef<B>) -> Self {
        Self(index)
    }
    fn sym_ref(&self) -> &SymRef<B> {
        &self.0
    }
}

impl<B: ModelStages> Display for Term<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data().str)
    }
}

impl<B: ModelStages> Debug for Term<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let term_data = self.data();
        let core = self.core();
        let coefficient = core
            .scalars
            .get(term_data.coef_scalar_ref.sym_idx)
            .symbol
            .clone();
        let variable = core.vars.get(term_data.var_ref.sym_idx).symbol.clone();

        f.debug_struct("Term")
            .field("str", &term_data.str)
            .field("coefficient", &coefficient)
            .field("variable", &variable)
            .finish()
    }
}

// data
impl<B: ModelStages> Term<B> {
    pub(crate) fn data(&self) -> &TermData<B> {
        self.core().terms.get(self.sym_idx())
    }
}
