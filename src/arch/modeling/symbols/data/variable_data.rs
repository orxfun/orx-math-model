use crate::modeling::model_data::ModelData;
use crate::modeling::reference::HasRef;
use crate::modeling::stages::ModelStages;
use crate::modeling::symbol_collections::symbol_data::SymbolData;
use crate::modeling::{
    stages::{Building, Built},
    symbols::{
        data::variable::{bounds::Bounds, vartype::VariableType},
        variable::Variable,
    },
};
use orx_imp_vec::prelude::NotSelfRefVecItem;
use std::rc::Rc;

#[derive(Clone)]
pub(crate) struct VariableData<const D: usize, B: ModelStages> {
    pub(crate) symbol: Variable<D, B>,
    pub(crate) bounds: Bounds<D>,
    pub(crate) vartype: VariableType,
    pub(crate) key: String,
    pub(crate) definition: String,
}

impl<const D: usize, B: ModelStages> SymbolData for VariableData<D, B> {
    type Symbol = Variable<D, B>;
    fn symbol(&self) -> Self::Symbol {
        self.symbol.clone()
    }
}
impl<const D: usize, B: ModelStages> NotSelfRefVecItem for VariableData<D, B> {}

impl<const D: usize> VariableData<D, Building> {
    pub(crate) fn build(&self, model: &Rc<ModelData<Built>>) -> VariableData<D, Built> {
        VariableData {
            symbol: Variable::new(self.symbol.sym_ref().build(model.clone())),
            bounds: self.bounds.clone(),
            vartype: self.vartype,
            key: self.key.clone(),
            definition: self.definition.clone(),
        }
    }
}
