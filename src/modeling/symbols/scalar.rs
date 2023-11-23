use super::data::scalar_data::{ScalarData, ScalarVariant};
use crate::modeling::reference::{HasRef, SymRef};
use crate::modeling::stages::ModelStages;
use crate::modeling::symbol_collections::symbol_collection::SymbolCollection;
use std::fmt::{Debug, Display};

#[derive(Clone, Copy)]
pub struct Scalar<B: ModelStages>(SymRef<B>);
impl<B: ModelStages> HasRef<B> for Scalar<B> {
    fn new(index: SymRef<B>) -> Self {
        Self(index)
    }
    fn sym_ref(&self) -> &SymRef<B> {
        &self.0
    }
}

impl<B: ModelStages> Display for Scalar<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let key = self.data().variant.str();
        write!(f, "{}", key)
    }
}

impl<B: ModelStages> Debug for Scalar<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variant = if matches!(&self.data().variant, ScalarVariant::Func { str: _, fun: _ }) {
            "Fun"
        } else {
            "Const"
        };

        f.debug_struct("Scalar")
            .field("key", &self.data().variant.str())
            .field("variant", &variant)
            .finish()
    }
}

// data
impl<B: ModelStages> Scalar<B> {
    pub(crate) fn data(&self) -> &ScalarData<B> {
        self.core().scalars.get(self.sym_idx())
    }
}
