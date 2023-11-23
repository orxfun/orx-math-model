use crate::modeling::model_data_ref::ModelDataRef;
use crate::modeling::stages::{Building, ModelStages};
use crate::modeling::symbol_collections::symbol_collection::SymbolCollection;
use crate::{
    data_structs::{expr_sum_refs::ExprSums, expr_term_refs::ExprTerms},
    modeling::{
        reference::HasRef,
        symbols::{
            data::expression_data::ExpressionData, expression::Expression, scalar::Scalar,
            sum::Sum, term::Term,
        },
    },
};

pub(crate) fn new_expr<Terms, Sums>(
    core_ref: ModelDataRef<Building>,
    sums: Sums,
    terms: Terms,
    constant: Option<Scalar<Building>>,
) -> Expression<Building>
where
    Terms: Iterator<Item = Term<Building>>,
    Sums: Iterator<Item = Sum<Building>>,
{
    let constant_scalar_ref = constant.map(|x| *x.sym_ref());
    let term_refs = terms.map(|x| *x.sym_ref()).into();
    let sum_refs = sums.map(|x| *x.sym_ref()).into();

    let symref = core_ref.new_symref_expression();
    let symbol = Expression::new(symref);

    let storage = ExpressionData {
        symbol,
        term_refs,
        sum_refs,
        constant_scalar_ref,
    };

    core_ref.core().expressions.push(storage)
}

pub(crate) fn get_sums_terms_and_constant<B: ModelStages>(
    expression: Expression<B>,
) -> (ExprSums<B>, ExprTerms<B>, Option<Scalar<B>>) {
    let core = expression.core();
    let data = core.expressions.get(expression.sym_idx());

    let sums = ExprSums::from_refs(core, &data.sum_refs);
    let terms = ExprTerms::from_indices(core, &data.term_refs);
    let constant = data
        .constant_scalar_ref
        .clone()
        .map(|x| core.scalars.get(x.sym_idx).symbol.clone());

    (sums, terms, constant)
}
