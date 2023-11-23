use crate::modeling::ops::expr::core::{get_sums_terms_and_constant, new_expr};
use crate::modeling::stages::Building;
use crate::modeling::{reference::HasRef, symbols::expression::Expression};
use std::ops::Neg;

impl Neg for Expression<Building> {
    type Output = Expression<Building>;
    fn neg(self) -> Self::Output {
        let (sums, terms, constant) = get_sums_terms_and_constant(self);

        let neg_sums = sums.iter().map(|sum| -sum);
        let neg_terms = terms.iter().map(|term| -term);
        let neg_constant = constant.map(|c| -c);

        new_expr(self.sym_ref().core_ref, neg_sums, neg_terms, neg_constant)
    }
}
