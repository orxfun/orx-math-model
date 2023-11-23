use super::{constraint_expression::ConstraintExpression, forall::ForAll};
use crate::modeling::model_data::ModelData;
use crate::modeling::reference::HasRef;
use crate::modeling::stages::{Building, Built, ModelStages};
use crate::modeling::symbol_collections::symbol_data::SymbolData;
use crate::modeling::symbols::constraint::Constraint;
use orx_imp_vec::prelude::NotSelfRefVecItem;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Clone, Copy, Debug)]
pub enum ConstraintRelation {
    Leq,
    Geq,
    Eq,
}
impl Display for ConstraintRelation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Leq => "<=",
                Self::Geq => ">=",
                Self::Eq => "=",
            }
        )
    }
}

pub(crate) struct ConstraintData<B: ModelStages> {
    pub(crate) symbol: Constraint<B>,
    pub(crate) constraint_expression: ConstraintExpression<B>,
    pub(crate) forall: ForAll<B>,
}
impl<B: ModelStages> SymbolData for ConstraintData<B> {
    type Symbol = Constraint<B>;
    fn symbol(&self) -> Self::Symbol {
        self.symbol.clone()
    }
}
impl<B: ModelStages> NotSelfRefVecItem for Constraint<B> {}

impl ConstraintData<Building> {
    pub(crate) fn build(&self, model: &Rc<ModelData<Built>>) -> ConstraintData<Built> {
        ConstraintData {
            symbol: Constraint::new(self.symbol.sym_ref().build(model.clone())),
            constraint_expression: self.constraint_expression.build(model),
            forall: self.forall.build(model),
        }
    }
}
