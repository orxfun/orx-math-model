use crate::{
    data_structs::sum_over_set_refs::SumOverSetRefs, modeling::stages::Building,
    modeling::symbols::expression::Expression,
};

pub struct SumExpression {
    pub(crate) sum_over_set_refs: SumOverSetRefs<Building>,
    pub(crate) expression: Expression<Building>,
}
impl SumExpression {
    pub(crate) fn new(
        sum_over_set_refs: SumOverSetRefs<Building>,
        expression: Expression<Building>,
    ) -> Self {
        Self {
            expression,
            sum_over_set_refs,
        }
    }
}
