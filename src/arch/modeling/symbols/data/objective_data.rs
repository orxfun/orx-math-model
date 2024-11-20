use super::objective_direction::ObjectiveDirection;
use crate::modeling::model_data::ModelData;
use crate::modeling::reference::HasRef;
use crate::modeling::stages::{Building, Built, ModelStages};
use crate::modeling::symbol_collections::symbol_data::SymbolData;
use crate::modeling::{reference::SymRef, symbols::objective::Objective};
use orx_imp_vec::prelude::NotSelfRefVecItem;
use std::rc::Rc;

pub(crate) struct ObjectiveData<B: ModelStages> {
    pub(crate) symbol: Objective<B>,
    pub(crate) direction: ObjectiveDirection,
    pub(crate) expression_ref: SymRef<B>,
}
impl<B: ModelStages> SymbolData for ObjectiveData<B> {
    type Symbol = Objective<B>;
    fn symbol(&self) -> Self::Symbol {
        self.symbol.clone()
    }
}
impl<B: ModelStages> NotSelfRefVecItem for Objective<B> {}

impl ObjectiveData<Building> {
    pub(crate) fn build(&self, model: &Rc<ModelData<Built>>) -> ObjectiveData<Built> {
        ObjectiveData {
            symbol: Objective::new(self.symbol.sym_ref().build(model.clone())),
            direction: self.direction,
            expression_ref: self.expression_ref.build(model.clone()),
        }
    }
}
