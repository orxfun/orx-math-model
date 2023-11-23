use crate::modeling::model_data::ModelData;
use crate::modeling::reference::HasRef;
use crate::modeling::stages::{Building, Built, ModelStages};
use crate::modeling::symbol_collections::symbol_data::SymbolData;
use crate::{
    data_structs::var_scalar_refs::VarScalarRefs,
    modeling::{reference::SymRef, symbols::var::Var},
};
use orx_imp_vec::prelude::NotSelfRefVecItem;
use std::rc::Rc;

#[derive(Clone)]
pub(crate) struct VarData<B: ModelStages> {
    pub(crate) symbol: Var<B>,
    pub(crate) variable_ref: SymRef<B>,
    pub(crate) scalar_refs: VarScalarRefs<B>,
    pub(crate) str: String,
}
impl<B: ModelStages> SymbolData for VarData<B> {
    type Symbol = Var<B>;
    fn symbol(&self) -> Self::Symbol {
        self.symbol.clone()
    }
}
impl<B: ModelStages> NotSelfRefVecItem for Var<B> {}

impl<B: ModelStages> VarData<B> {
    pub(crate) fn variable_dimension(&self) -> usize {
        self.scalar_refs.len()
    }
}

impl VarData<Building> {
    pub(crate) fn build(&self, model: &Rc<ModelData<Built>>) -> VarData<Built> {
        VarData {
            symbol: Var::new(self.symbol.sym_ref().build(model.clone())),
            variable_ref: self.variable_ref.build(model.clone()),
            scalar_refs: self.scalar_refs.build(model),
            str: self.str.clone(),
        }
    }
}
