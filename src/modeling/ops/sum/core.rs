use super::sum_expression::SumExpression;
use crate::modeling::stages::{Building, ModelStages};
use crate::modeling::symbol_collections::symbol_collection::SymbolCollection;
use crate::{
    data_structs::sum_over_set_refs::SumOverSetRefs,
    modeling::{
        reference::HasRef,
        symbols::{data::sum_data::SumData, expression::Expression, sum::Sum},
    },
};

pub(crate) fn new_sum(sum_expression: SumExpression) -> Sum<Building> {
    let core_ref = sum_expression.expression.sym_ref().core_ref;
    let symref = core_ref.new_symref_sum();
    let symbol = Sum::new(symref);
    let sum_over_set_refs = sum_expression.sum_over_set_refs;
    let expression_ref = *sum_expression.expression.sym_ref();
    let storage = SumData {
        symbol,
        sum_over_set_refs,
        expression_ref,
    };
    core_ref.core().sums.push(storage)
}

pub(crate) fn get_sumoversets_and_expr<B: ModelStages>(
    sum: Sum<B>,
) -> (SumOverSetRefs<B>, Expression<B>) {
    let core = sum.core();
    let data = core.sums.get(sum.sym_idx());
    let sum_over_set_refs = data.sum_over_set_refs.clone();
    let expression = core
        .expressions
        .get(data.expression_ref.sym_idx)
        .symbol
        .clone();

    (sum_over_set_refs, expression)
}
