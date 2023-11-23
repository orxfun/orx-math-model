use crate::modeling::ops::expr::core::new_expr;
use crate::modeling::stages::Building;
use crate::modeling::{
    model_data_ref::ModelDataRef,
    symbols::{conversions::temp_expr::TempExpr, expression::Expression, scalar::Scalar},
};

pub(super) fn add(
    core_ref: ModelDataRef<Building>,
    lhs: TempExpr,
    rhs: TempExpr,
) -> Expression<Building> {
    let constant = merge_constants(lhs.constant, rhs.constant, false);
    let terms = lhs.terms.iter().chain(rhs.terms.iter());
    let sums = lhs.sums.iter().chain(rhs.sums.iter());
    new_expr(core_ref, sums, terms, constant)
}

pub(super) fn sub(
    core_ref: ModelDataRef<Building>,
    lhs: TempExpr,
    rhs: TempExpr,
) -> Expression<Building> {
    let constant = merge_constants(lhs.constant, rhs.constant, false);
    let terms = lhs.terms.iter().chain(rhs.terms.iter().map(|x| -x));
    let sums = lhs.sums.iter().chain(rhs.sums.iter().map(|x| -x));
    new_expr(core_ref, sums, terms, constant)
}

fn merge_constants(
    lhs_const: Option<Scalar<Building>>,
    rhs_const: Option<Scalar<Building>>,
    negate_rhs: bool,
) -> Option<Scalar<Building>> {
    let rhs_const = if negate_rhs {
        rhs_const.map(|x| -x)
    } else {
        rhs_const
    };
    match (lhs_const, rhs_const) {
        (None, None) => None,
        (Some(l), None) => Some(l),
        (None, Some(r)) => Some(r),
        (Some(l), Some(r)) => Some(l + r),
    }
}
