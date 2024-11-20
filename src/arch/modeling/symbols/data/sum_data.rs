use crate::modeling::model_data::ModelData;
use crate::modeling::reference::HasRef;
use crate::modeling::stages::{Building, Built, ModelStages};
use crate::modeling::symbol_collections::symbol_data::SymbolData;
use crate::{
    data_structs::sum_over_set_refs::SumOverSetRefs,
    modeling::{reference::SymRef, symbols::sum::Sum},
};
use orx_imp_vec::prelude::NotSelfRefVecItem;
use std::rc::Rc;

pub(crate) struct SumData<B: ModelStages> {
    pub(crate) symbol: Sum<B>,
    pub(crate) sum_over_set_refs: SumOverSetRefs<B>,
    pub(crate) expression_ref: SymRef<B>,
}
impl<B: ModelStages> SymbolData for SumData<B> {
    type Symbol = Sum<B>;
    fn symbol(&self) -> Self::Symbol {
        self.symbol.clone()
    }
}
impl<B: ModelStages> NotSelfRefVecItem for Sum<B> {}

impl SumData<Building> {
    pub(crate) fn build(&self, model: &Rc<ModelData<Built>>) -> SumData<Built> {
        SumData {
            symbol: Sum::new(self.symbol.sym_ref().build(model.clone())),
            sum_over_set_refs: self.sum_over_set_refs.build(model),
            expression_ref: self.expression_ref.build(model.clone()),
        }
    }
}
