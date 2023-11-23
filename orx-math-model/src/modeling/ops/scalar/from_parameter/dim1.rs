use crate::modeling::stages::Building;
use crate::modeling::symbol_collections::{scalars::Scalars, symbol_collection::SymbolCollection};
use crate::{
    data_structs::current_set_indices::CurrentSetElements,
    modeling::{
        reference::HasRef,
        symbols::{
            conversions::scalar::ToScalar,
            data::scalar_data::{ScalarData, ScalarVariant},
            parameter::Parameter,
            scalar::Scalar,
        },
    },
    str_expr,
};
use std::{ops::Index, rc::Rc};

impl<S: ToScalar> Index<S> for Parameter<1, Building> {
    type Output = Scalar<Building>;
    fn index(&self, indices: S) -> &Self::Output {
        let fun_par = self.data().fun.clone();

        let scalar_indices = [indices.to_scalar_from_host(self.sym_ref())];

        let scalars = &self.core().scalars;

        let keys = scalar_indices.map(|s| scalars.get(s.sym_idx()).variant.str());
        let str = str_expr::index(&self.data().key, keys);

        let variants = get_variants(scalars, &scalar_indices);

        let fun = Rc::new(move |indices: &CurrentSetElements| {
            let par_indices = [variants[0].get_value_as_index(indices)];
            fun_par.value(par_indices)
        });

        let variant = ScalarVariant::Func { str, fun };
        let symref = self.sym_ref().core_ref.new_symref_scalar();
        let symbol = Scalar::new(symref);
        let storage = ScalarData { symbol, variant };
        _ = scalars.push(storage);

        &scalars.get(symref.sym_idx).symbol
    }
}

pub(crate) fn get_variants<const D: usize>(
    scalars: &Scalars<Building>,
    scalar_indices: &[Scalar<Building>; D],
) -> [ScalarVariant; D] {
    scalar_indices.map(|s| scalars.get(s.sym_idx()).variant.clone())
}
