use super::{constraint_rhs::ConstraintRhs, temp_expr::ToTempExpr};
use crate::{
    modeling::stages::Building,
    modeling::{
        reference::HasRef,
        symbols::{
            data::{
                constraint_data::ConstraintRelation, constraint_expression::ConstraintExpression,
            },
            expression::Expression,
            scalar::Scalar,
            set::Set,
            sum::Sum,
            term::Term,
            var::Var,
            variable::Variable,
        },
    },
};

fn new_constraint_expression<L: ConstraintLhs, R: ConstraintRhs>(
    lhs: &L,
    rhs: R,
    relation: ConstraintRelation,
) -> ConstraintExpression<Building> {
    let lhs = lhs.to_lhs_expression();
    let core_ref = lhs.sym_ref().core_ref;
    let rhs = rhs.to_rhs_expression(core_ref);
    let lhs_expression_ref = *lhs.sym_ref();
    let rhs_expression_ref = *rhs.sym_ref();
    ConstraintExpression {
        core_ref,
        lhs_expression_ref,
        rhs_expression_ref,
        relation,
    }
}

pub trait ConstraintLhs: Sized {
    fn to_lhs_expression(&self) -> Expression<Building>;

    fn equal_to<R: ConstraintRhs>(&self, rhs: R) -> ConstraintExpression<Building> {
        new_constraint_expression(self, rhs, ConstraintRelation::Eq)
    }

    fn less_than_or_eq<R: ConstraintRhs>(&self, rhs: R) -> ConstraintExpression<Building> {
        new_constraint_expression(self, rhs, ConstraintRelation::Leq)
    }

    fn greater_than_or_eq<R: ConstraintRhs>(&self, rhs: R) -> ConstraintExpression<Building> {
        new_constraint_expression(self, rhs, ConstraintRelation::Geq)
    }
}

impl ConstraintLhs for Expression<Building> {
    fn to_lhs_expression(&self) -> Expression<Building> {
        *self
    }
}
impl ConstraintLhs for Sum<Building> {
    fn to_lhs_expression(&self) -> Expression<Building> {
        self.add_to_model_as_expr(self.sym_ref().core_ref)
    }
}
impl ConstraintLhs for Term<Building> {
    fn to_lhs_expression(&self) -> Expression<Building> {
        self.add_to_model_as_expr(self.sym_ref().core_ref)
    }
}
impl ConstraintLhs for Var<Building> {
    fn to_lhs_expression(&self) -> Expression<Building> {
        self.add_to_model_as_expr(self.sym_ref().core_ref)
    }
}
impl ConstraintLhs for Variable<0, Building> {
    fn to_lhs_expression(&self) -> Expression<Building> {
        self.add_to_model_as_expr(self.sym_ref().core_ref)
    }
}
impl ConstraintLhs for Scalar<Building> {
    fn to_lhs_expression(&self) -> Expression<Building> {
        self.add_to_model_as_expr(self.sym_ref().core_ref)
    }
}
impl ConstraintLhs for Set<Building> {
    fn to_lhs_expression(&self) -> Expression<Building> {
        self.add_to_model_as_expr(self.sym_ref().core_ref)
    }
}
