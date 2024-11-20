use super::data::variable_data::VariableData;
use crate::modeling::{
    reference::{HasRef, SymRef},
    stages::ModelStages,
    symbol_collections::symbol_collection::SymbolCollection,
};
use std::fmt::{Debug, Display};

#[derive(Clone, Copy)]
pub struct Variable<const D: usize, B: ModelStages>(SymRef<B>);

impl<const D: usize, B: ModelStages> HasRef<B> for Variable<D, B> {
    fn new(index: SymRef<B>) -> Self {
        Self(index)
    }
    fn sym_ref(&self) -> &SymRef<B> {
        &self.0
    }
}

impl<const D: usize, B: ModelStages> Display for Variable<D, B>
where
    Variable<D, B>: HasRef<B>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key = self.core().variables.get_key(D, self.sym_idx());
        write!(f, "{}", key)
    }
}

impl<const D: usize, B: ModelStages> Debug for Variable<D, B>
where
    Variable<D, B>: HasRef<B>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let idx = self.sym_idx();
        let variables = &self.core().variables;
        let key = variables.get_key(D, idx);
        let definition = variables.get_definition(D, idx);
        let vartype = variables.get_vartype(D, idx);
        let bounds = variables.get_bounds_str(D, idx);

        f.debug_struct("Variable")
            .field("dimension", &D)
            .field("key", &key)
            .field("definition", &definition)
            .field("type", &vartype)
            .field("bounds", &bounds)
            .finish()
    }
}

// data
impl<B: ModelStages> Variable<0, B> {
    pub(crate) fn data(&self) -> &VariableData<0, B> {
        self.core().variables.dim0.get(self.sym_idx())
    }
}
impl<B: ModelStages> Variable<1, B> {
    pub(crate) fn data(&self) -> &VariableData<1, B> {
        self.core().variables.dim1.get(self.sym_idx())
    }
}
impl<B: ModelStages> Variable<2, B> {
    pub(crate) fn data(&self) -> &VariableData<2, B> {
        self.core().variables.dim2.get(self.sym_idx())
    }
}

impl<B: ModelStages> Variable<3, B> {
    pub(crate) fn data(&self) -> &VariableData<3, B> {
        self.core().variables.dim3.get(self.sym_idx())
    }
}
impl<B: ModelStages> Variable<4, B> {
    pub(crate) fn data(&self) -> &VariableData<4, B> {
        self.core().variables.dim4.get(self.sym_idx())
    }
}
