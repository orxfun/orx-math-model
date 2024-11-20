use crate::data_structs::funvec::FunVec;
use crate::modeling::model_data::ModelData;
use crate::modeling::reference::HasRef;
use crate::modeling::stages::{Building, Built, ModelStages};
use crate::modeling::symbol_collections::symbol_data::SymbolData;
use crate::modeling::symbols::parameter::Parameter;
use crate::numerics::num::RealNum;
use orx_imp_vec::prelude::NotSelfRefVecItem;
use std::rc::Rc;

pub(crate) struct ParameterData<const D: usize, B: ModelStages> {
    pub(crate) symbol: Parameter<D, B>,
    pub(crate) fun: Rc<dyn FunVec<D, RealNum>>,
    pub(crate) key: String,
    pub(crate) definition: String,
}

impl<const D: usize, B: ModelStages> SymbolData for ParameterData<D, B> {
    type Symbol = Parameter<D, B>;
    fn symbol(&self) -> Self::Symbol {
        self.symbol.clone()
    }
}
impl<const D: usize, B: ModelStages> NotSelfRefVecItem for ParameterData<D, B> {}

impl<const D: usize> ParameterData<D, Building> {
    pub(crate) fn build(&self, model: &Rc<ModelData<Built>>) -> ParameterData<D, Built> {
        ParameterData {
            symbol: Parameter::new(self.symbol.sym_ref().build(model.clone())),
            fun: self.fun.clone(),
            key: self.key.clone(),
            definition: self.definition.clone(),
        }
    }
}
