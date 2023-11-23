use crate::modeling::model_data::ModelData;
use crate::modeling::reference::HasRef;
use crate::modeling::stages::{Building, Built, ModelStages};
use crate::modeling::symbol_collections::symbol_data::SymbolData;
use crate::{
    data_structs::{expr_sum_refs::ExprSumRefs, expr_term_refs::ExprTermRefs},
    modeling::{reference::SymRef, symbols::expression::Expression},
};
use orx_imp_vec::prelude::NotSelfRefVecItem;
use std::rc::Rc;

pub(crate) struct ExpressionData<B: ModelStages> {
    pub(crate) symbol: Expression<B>,
    pub(crate) constant_scalar_ref: Option<SymRef<B>>,
    pub(crate) term_refs: ExprTermRefs<B>,
    pub(crate) sum_refs: ExprSumRefs<B>,
}
impl<B: ModelStages> SymbolData for ExpressionData<B> {
    type Symbol = Expression<B>;
    fn symbol(&self) -> Self::Symbol {
        self.symbol.clone()
    }
}
impl<B: ModelStages> NotSelfRefVecItem for Expression<B> {}

impl ExpressionData<Building> {
    pub(crate) fn build(&self, model: &Rc<ModelData<Built>>) -> ExpressionData<Built> {
        ExpressionData {
            symbol: Expression::new(self.symbol.sym_ref().build(model.clone())),
            constant_scalar_ref: self.constant_scalar_ref.map(|x| x.build(model.clone())),
            term_refs: self.term_refs.build(model),
            sum_refs: self.sum_refs.build(model),
        }
    }
}
