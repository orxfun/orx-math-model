use crate::{
    modeling::{
        ops::expr::core::get_sums_terms_and_constant,
        reference::{HasRef, SymRef},
        stages::ModelStages,
        symbol_collections::symbol_collection::SymbolCollection,
    },
    str_expr,
};
use std::fmt::{Debug, Display};

use super::data::expression_data::ExpressionData;

#[derive(Clone, Copy)]
pub struct Expression<B: ModelStages>(SymRef<B>);
impl<B: ModelStages> HasRef<B> for Expression<B> {
    fn new(index: SymRef<B>) -> Self {
        Self(index)
    }
    fn sym_ref(&self) -> &SymRef<B> {
        &self.0
    }
}

impl<B: ModelStages> Display for Expression<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (sums, terms, constant) = get_sums_terms_and_constant(self.clone());

        let sums_str = sums.iter().map(|t| t.to_string());
        let terms_str = terms.iter().map(|t| t.to_string());
        let terms_and_sums = sums_str.chain(terms_str);

        let expr = if let Some(constant) = constant {
            str_expr::join_terms(terms_and_sums.chain([constant.to_string()]))
        } else {
            str_expr::join_terms(terms_and_sums)
        };

        write!(f, "{}", expr)
    }
}

impl<B: ModelStages> Debug for Expression<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (sums, terms, constant) = get_sums_terms_and_constant(self.clone());
        let terms: Vec<_> = terms.iter().collect();
        let sums: Vec<_> = sums.iter().collect();

        f.debug_struct("Expression")
            .field("key", &self.to_string())
            .field("number_of_sums", &sums.len())
            .field("number_of_terms", &terms.len())
            .field("sums", &sums)
            .field("terms", &terms)
            .field("constant", &constant)
            .finish()
    }
}

// data
impl<B: ModelStages> Expression<B> {
    pub(crate) fn data(&self) -> &ExpressionData<B> {
        self.core().expressions.get(self.sym_idx())
    }
}
