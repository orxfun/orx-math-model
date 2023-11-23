use crate::modeling::{
    reference::{HasRef, SymRef},
    stages::ModelStages,
    symbol_collections::symbol_collection::SymbolCollection,
};
use std::fmt::{Debug, Display};

use super::data::parameter_data::ParameterData;

#[derive(Clone, Copy)]
pub struct Parameter<const D: usize, B: ModelStages>(SymRef<B>);

impl<const D: usize, B: ModelStages> HasRef<B> for Parameter<D, B> {
    fn new(index: SymRef<B>) -> Self {
        Self(index)
    }
    fn sym_ref(&self) -> &SymRef<B> {
        &self.0
    }
}

impl<const D: usize, B: ModelStages> Display for Parameter<D, B>
where
    Parameter<D, B>: HasRef<B>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key = self.core().parameters.get_key(D, self.sym_idx());
        write!(f, "{}", key)
    }
}

impl<const D: usize, B: ModelStages> Debug for Parameter<D, B>
where
    Parameter<D, B>: HasRef<B>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key = self.core().parameters.get_key(D, self.sym_idx());
        let definition = self.core().parameters.get_definition(D, self.sym_idx());

        f.debug_struct("Parameter")
            .field("dimension", &D)
            .field("key", &key)
            .field("definition", &definition)
            .finish()
    }
}

// data
impl<B: ModelStages> Parameter<0, B> {
    pub(crate) fn data(&self) -> &ParameterData<0, B> {
        self.core().parameters.dim0.get(self.sym_idx())
    }
}
impl<B: ModelStages> Parameter<1, B> {
    pub(crate) fn data(&self) -> &ParameterData<1, B> {
        self.core().parameters.dim1.get(self.sym_idx())
    }
}
impl<B: ModelStages> Parameter<2, B> {
    pub(crate) fn data(&self) -> &ParameterData<2, B> {
        self.core().parameters.dim2.get(self.sym_idx())
    }
}

impl<B: ModelStages> Parameter<3, B> {
    pub(crate) fn data(&self) -> &ParameterData<3, B> {
        self.core().parameters.dim3.get(self.sym_idx())
    }
}
impl<B: ModelStages> Parameter<4, B> {
    pub(crate) fn data(&self) -> &ParameterData<4, B> {
        self.core().parameters.dim4.get(self.sym_idx())
    }
}
