use super::dim1::get_variants;
use crate::{
    data_structs::current_set_indices::CurrentSetElements,
    modeling::stages::Building,
    modeling::{
        reference::HasRef,
        symbol_collections::symbol_collection::SymbolCollection,
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

impl<S1, S2, S3, S4> Index<(S1, S2, S3, S4)> for Parameter<4, Building>
where
    S1: ToScalar,
    S2: ToScalar,
    S3: ToScalar,
    S4: ToScalar,
{
    type Output = Scalar<Building>;
    fn index(&self, indices: (S1, S2, S3, S4)) -> &Self::Output {
        let fun_par = self.data().fun.clone();

        let scalar_indices = [
            indices.0.to_scalar_from_host(self.sym_ref()),
            indices.1.to_scalar_from_host(self.sym_ref()),
            indices.2.to_scalar_from_host(self.sym_ref()),
            indices.3.to_scalar_from_host(self.sym_ref()),
        ];

        let scalars = &self.core().scalars;

        let keys = scalar_indices.map(|s| scalars.get(s.sym_idx()).variant.str());
        let str = str_expr::index(&self.data().key, keys);

        let variants = get_variants(scalars, &scalar_indices);

        let fun = Rc::new(move |indices: &CurrentSetElements| {
            let par_indices = [
                variants[0].get_value_as_index(indices),
                variants[1].get_value_as_index(indices),
                variants[2].get_value_as_index(indices),
                variants[3].get_value_as_index(indices),
            ];
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
