use super::set::set_elements::SetElements;
use crate::modeling::model_data::ModelData;
use crate::modeling::reference::HasRef;
use crate::modeling::stages::{Building, Built, ModelStages};
use crate::modeling::symbol_collections::symbol_data::SymbolData;
use crate::modeling::{reference::SymRef, symbols::set::Set};
use orx_imp_vec::prelude::NotSelfRefVecItem;
use std::rc::Rc;

#[derive(Clone)]
pub(crate) struct SetData<B: ModelStages> {
    pub(crate) symbol: Set<B>,
    pub(crate) elements: SetElements,
    pub(crate) scalar_ref: SymRef<B>,
    pub(crate) key: String,
    pub(crate) definition: String,
}

impl<B: ModelStages> SymbolData for SetData<B> {
    type Symbol = Set<B>;
    fn symbol(&self) -> Self::Symbol {
        self.symbol.clone()
    }
}
impl<B: ModelStages> NotSelfRefVecItem for Set<B> {}

impl SetData<Building> {
    pub(crate) fn build(&self, model: &Rc<ModelData<Built>>) -> SetData<Built> {
        SetData {
            symbol: Set::new(self.symbol.sym_ref().build(model.clone())),
            elements: self.elements.clone(),
            scalar_ref: self.scalar_ref.build(model.clone()),
            key: self.key.clone(),
            definition: self.definition.clone(),
        }
    }
}
