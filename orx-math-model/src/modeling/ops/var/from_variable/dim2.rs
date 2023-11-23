use super::core::new_var;
use crate::{
    modeling::stages::Building,
    modeling::{
        reference::HasRef,
        symbols::{conversions::scalar::ToScalar, var::Var, variable::Variable},
    },
};
use std::ops::Index;

impl<S1, S2> Index<(S1, S2)> for Variable<2, Building>
where
    S1: ToScalar,
    S2: ToScalar,
{
    type Output = Var<Building>;
    fn index(&self, indices: (S1, S2)) -> &Self::Output {
        let scalar_indices = [
            indices.0.to_scalar_from_host(self.sym_ref()),
            indices.1.to_scalar_from_host(self.sym_ref()),
        ];

        new_var(self, scalar_indices)
    }
}
