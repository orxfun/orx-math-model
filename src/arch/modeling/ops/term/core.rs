use crate::modeling::stages::Building;
use crate::modeling::symbol_collections::symbol_collection::SymbolCollection;
use crate::{
    modeling::{
        reference::HasRef,
        symbols::{data::term_data::TermData, scalar::Scalar, term::Term, var::Var},
    },
    numerics::ops::Ops,
    str_expr,
};

pub(super) fn new_term(var: Var<Building>, scalar: Scalar<Building>) -> Term<Building> {
    let core = var.core();
    let coef_scalar_ref = *scalar.sym_ref();
    let var_ref = *var.sym_ref();
    let sca_str = core.scalars.get(scalar.sym_idx()).variant.str();
    let var_str = &core.vars.get(var.sym_idx()).str;
    let str = str_expr::binary_ops(Ops::Mul, sca_str, var_str);

    let symref = var.sym_ref().core_ref.new_symref_term();
    let symbol = Term::new(symref);
    let storage = TermData {
        coef_scalar_ref,
        var_ref,
        str,
        symbol,
    };
    core.terms.push(storage)
}

pub(super) fn get_term_coef_var(term: Term<Building>) -> (Scalar<Building>, Var<Building>) {
    let core = term.core();

    let term_data = core.terms.get(term.sym_idx());

    let old_coef_idx = term_data.coef_scalar_ref.sym_idx;
    let coef = core.scalars.get(old_coef_idx).symbol;

    let var = core.vars.get(term_data.var_ref.sym_idx).symbol;

    (coef, var)
}
