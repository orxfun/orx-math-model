use crate::modeling::model_data::ModelData;
use crate::modeling::reference::HasRef;
use crate::modeling::stages::{Building, Built, ModelStages};
use crate::modeling::symbol_collections::symbol_data::SymbolData;
use crate::modeling::{reference::SymRef, symbols::term::Term};
use orx_imp_vec::prelude::NotSelfRefVecItem;
use std::rc::Rc;

pub(crate) struct TermData<B: ModelStages> {
    pub(crate) symbol: Term<B>,
    pub(crate) coef_scalar_ref: SymRef<B>,
    pub(crate) var_ref: SymRef<B>,
    pub(crate) str: String,
}

impl<B: ModelStages> SymbolData for TermData<B> {
    type Symbol = Term<B>;
    fn symbol(&self) -> Self::Symbol {
        self.symbol.clone()
    }
}
impl<B: ModelStages> NotSelfRefVecItem for Term<B> {}

impl TermData<Building> {
    pub(crate) fn build(&self, model: &Rc<ModelData<Built>>) -> TermData<Built> {
        TermData {
            symbol: Term::new(self.symbol.sym_ref().build(model.clone())),
            coef_scalar_ref: self.coef_scalar_ref.build(model.clone()),
            var_ref: self.var_ref.build(model.clone()),
            str: self.str.clone(),
        }
    }
}
