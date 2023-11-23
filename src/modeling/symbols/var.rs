use super::{data::var_data::VarData, variable::Variable};
use crate::modeling::stages::ModelStages;
use crate::modeling::{
    model_data::ModelData,
    reference::{HasRef, SymRef},
    symbol_collections::symbol_collection::SymbolCollection,
};
use std::fmt::{Debug, Display};

#[derive(Clone, Copy)]
pub struct Var<B: ModelStages>(SymRef<B>);
impl<B: ModelStages> HasRef<B> for Var<B> {
    fn new(index: SymRef<B>) -> Self {
        Self(index)
    }
    fn sym_ref(&self) -> &SymRef<B> {
        &self.0
    }
}

impl<B: ModelStages> Display for Var<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data().str)
    }
}

impl<B: ModelStages> Debug for Var<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let core = self.core();
        let data = self.data();
        let var_idx = data.variable_ref.sym_idx;
        let dim = data.variable_dimension();
        match dim {
            0 => debug(
                f,
                core,
                data,
                core.variables.dim0.get(var_idx).symbol.clone(),
            ),
            1 => debug(
                f,
                core,
                data,
                core.variables.dim1.get(var_idx).symbol.clone(),
            ),
            2 => debug(
                f,
                core,
                data,
                core.variables.dim2.get(var_idx).symbol.clone(),
            ),
            3 => debug(
                f,
                core,
                data,
                core.variables.dim3.get(var_idx).symbol.clone(),
            ),
            4 => debug(
                f,
                core,
                data,
                core.variables.dim4.get(var_idx).symbol.clone(),
            ),
            _ => panic!("nono"),
        }
    }
}

fn debug<const D: usize, B: ModelStages>(
    f: &mut std::fmt::Formatter<'_>,
    core: &ModelData<B>,
    data: &VarData<B>,
    variable: Variable<D, B>,
) -> std::fmt::Result
where
    Variable<D, B>: HasRef<B>,
{
    let scalars: Vec<_> = data
        .scalar_refs
        .as_slice()
        .iter()
        .map(|sca_ref| core.scalars.get(sca_ref.sym_idx).symbol.clone())
        .collect();

    f.debug_struct("Var")
        .field("str", &data.str)
        .field("variable", &variable)
        .field("indices", &scalars)
        .finish()
}

// data
impl<B: ModelStages> Var<B> {
    pub(crate) fn data(&self) -> &VarData<B> {
        self.core().vars.get(self.sym_idx())
    }
}
