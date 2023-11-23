use crate::modeling::ops::expr::core::{get_sums_terms_and_constant, new_expr};
use crate::modeling::stages::Building;
use crate::modeling::{
    reference::HasRef,
    symbols::{conversions::scalar::ToScalar, expression::Expression},
};
use std::ops::Div;

impl<S: ToScalar> Div<S> for Expression<Building> {
    type Output = Expression<Building>;
    fn div(self, rhs: S) -> Self::Output {
        let div = rhs.to_scalar_from_host(self.sym_ref());

        let (sums, terms, constant) = get_sums_terms_and_constant(self);

        let new_constant = constant.map(|x| x / div);
        let new_terms = terms.iter().map(|x| x / div);
        let new_sums = sums.iter().map(|x| x / div);

        new_expr(self.sym_ref().core_ref, new_sums, new_terms, new_constant)
    }
}
