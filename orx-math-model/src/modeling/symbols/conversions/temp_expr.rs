use crate::modeling::model_data_ref::ModelDataRef;
use crate::modeling::stages::Building;
use crate::modeling::symbol_collections::symbol_collection::SymbolCollection;
use crate::{
    data_structs::{expr_sum_refs::ExprSums, expr_term_refs::ExprTerms},
    modeling::{
        ops::expr::core::new_expr,
        symbols::{
            conversions::{scalar::ToScalar, var::ToVar},
            expression::Expression,
            scalar::Scalar,
            sum::Sum,
            term::Term,
            var::Var,
            variable::Variable,
        },
    },
};

pub(crate) struct TempExpr {
    pub(crate) terms: ExprTerms<Building>,
    pub(crate) sums: ExprSums<Building>,
    pub(crate) constant: Option<Scalar<Building>>,
}
impl TempExpr {
    pub(crate) fn to_expr(&self, core_ref: ModelDataRef<Building>) -> Expression<Building> {
        new_expr(core_ref, self.sums.iter(), self.terms.iter(), self.constant)
    }
}

pub(crate) trait ToTempExpr {
    fn to_temp_expr(&self, core_ref: ModelDataRef<Building>) -> TempExpr;
    fn add_to_model_as_expr(&self, core_ref: ModelDataRef<Building>) -> Expression<Building> {
        self.to_temp_expr(core_ref).to_expr(core_ref)
    }
}

impl ToTempExpr for Expression<Building> {
    fn to_temp_expr(&self, core_ref: ModelDataRef<Building>) -> TempExpr {
        let data = self.data();
        let sums: ExprSums<Building> = data
            .sum_refs
            .iter()
            .map(|x| core_ref.core().sums.get(x.sym_idx).symbol)
            .into();
        let terms: ExprTerms<Building> = data
            .term_refs
            .iter()
            .map(|x| core_ref.core().terms.get(x.sym_idx).symbol)
            .into();
        let constant = data
            .constant_scalar_ref
            .map(|x| core_ref.core().scalars.get(x.sym_idx).symbol);
        TempExpr {
            sums,
            terms,
            constant,
        }
    }
}
impl ToTempExpr for Sum<Building> {
    fn to_temp_expr(&self, _: ModelDataRef<Building>) -> TempExpr {
        let data = self.data();
        let constant = None;
        let terms = ExprTerms::empty();
        let sums = ExprSums::singleton(data.symbol);
        TempExpr {
            sums,
            terms,
            constant,
        }
    }
}
impl ToTempExpr for Term<Building> {
    fn to_temp_expr(&self, _: ModelDataRef<Building>) -> TempExpr {
        let data = self.data();
        let constant = None;
        let terms = ExprTerms::singleton(data.symbol);
        let sums = ExprSums::empty();
        TempExpr {
            sums,
            terms,
            constant,
        }
    }
}
impl ToTempExpr for Var<Building> {
    fn to_temp_expr(&self, _: ModelDataRef<Building>) -> TempExpr {
        let data = self.data();
        let constant = None;
        let terms = ExprTerms::singleton(1.0 * data.symbol);
        let sums = ExprSums::empty();
        TempExpr {
            sums,
            terms,
            constant,
        }
    }
}
impl ToTempExpr for Variable<0, Building> {
    fn to_temp_expr(&self, core_ref: ModelDataRef<Building>) -> TempExpr {
        let var = self.to_var();
        var.to_temp_expr(core_ref)
    }
}

impl<S: ToScalar> ToTempExpr for S {
    fn to_temp_expr(&self, core_ref: ModelDataRef<Building>) -> TempExpr {
        let constant = Some(self.to_scalar(core_ref));
        let terms = ExprTerms::empty();
        let sums = ExprSums::empty();
        TempExpr {
            sums,
            terms,
            constant,
        }
    }
}
