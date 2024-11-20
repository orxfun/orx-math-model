use super::constraint_data::ConstraintRelation;
use crate::modeling::model_data::ModelData;
use crate::modeling::model_data_ref::ModelDataRef;
use crate::modeling::stages::{Building, Built, ModelStages};
use crate::modeling::symbol_collections::symbol_collection::SymbolCollection;
use crate::{
    data_structs::forall_set_refs::ForAllSetRefs,
    modeling::{
        ops::constraint::new_constraint,
        reference::SymRef,
        symbols::{constraint::Constraint, data::forall::ForAll},
    },
};
use std::fmt::{Debug, Display};
use std::rc::Rc;

#[derive(Clone, Copy)]
pub struct ConstraintExpression<B: ModelStages> {
    pub(crate) core_ref: ModelDataRef<B>,
    pub(crate) lhs_expression_ref: SymRef<B>,
    pub(crate) rhs_expression_ref: SymRef<B>,
    pub(crate) relation: ConstraintRelation,
}

impl ConstraintExpression<Building> {
    pub(crate) fn as_single_constraint(&self) -> Constraint<Building> {
        let forall = ForAll {
            sets: ForAllSetRefs::empty(),
        };
        new_constraint(*self, forall)
    }

    pub(crate) fn build(&self, model: &Rc<ModelData<Built>>) -> ConstraintExpression<Built> {
        ConstraintExpression {
            core_ref: ModelDataRef {
                model_reference: Built(model.clone()),
            },
            lhs_expression_ref: self.lhs_expression_ref.build(model.clone()),
            rhs_expression_ref: self.rhs_expression_ref.build(model.clone()),
            relation: self.relation,
        }
    }
}

impl<B: ModelStages> Display for ConstraintExpression<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let expressions = &self.core_ref.core().expressions;
        let lhs = expressions
            .get(self.lhs_expression_ref.sym_idx)
            .symbol
            .clone();
        let rhs = expressions
            .get(self.rhs_expression_ref.sym_idx)
            .symbol
            .clone();

        write!(f, "{} {} {}", lhs, self.relation, rhs)
    }
}
impl<B: ModelStages> Debug for ConstraintExpression<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let expressions = &self.core_ref.core().expressions;
        let lhs = expressions
            .get(self.lhs_expression_ref.sym_idx)
            .symbol
            .clone();
        let rhs = expressions
            .get(self.rhs_expression_ref.sym_idx)
            .symbol
            .clone();

        f.debug_struct("ConstraintExpression")
            .field("relation", &self.relation)
            .field("lhs", &lhs)
            .field("rhs", &rhs)
            .finish()
    }
}
