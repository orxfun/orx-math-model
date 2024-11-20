use crate::modeling::stages::Building;
use crate::modeling::symbol_collections::symbol_collection::SymbolCollection;
use crate::{
    data_structs::var_scalar_refs::VarScalarRefs,
    modeling::{
        reference::HasRef,
        symbols::{data::var_data::VarData, scalar::Scalar, var::Var, variable::Variable},
    },
    str_expr,
};

pub(super) fn new_var<const D: usize>(
    variable: &Variable<D, Building>,
    scalar_indices: [Scalar<Building>; D],
) -> &Var<Building>
where
    Variable<D, Building>: HasRef<Building>,
{
    let core_ref = variable.sym_ref().core_ref;
    let core = core_ref.core();
    let vars = &core.vars;

    let variable_ref = *variable.sym_ref();
    let variable_key = core.variables.get_key(D, variable.sym_idx());

    let keys = scalar_indices.map(|s| core.scalars.get(s.sym_idx()).variant.str());
    let str = if D == 0 {
        variable_key.to_string()
    } else {
        str_expr::index(variable_key, keys)
    };

    let scalar_indices = VarScalarRefs::from_scalars(&scalar_indices);

    let symref = core_ref.new_symref_var();
    let symbol = Var::new(symref);
    let storage = VarData {
        symbol,
        variable_ref,
        scalar_refs: scalar_indices,
        str,
    };
    _ = vars.push(storage);

    let idx = symref.sym_idx;
    &variable.core().vars.get(idx).symbol
}
