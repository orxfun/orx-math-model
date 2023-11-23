use super::core::new_var;
use crate::{
    modeling::stages::Building,
    modeling::{
        reference::HasRef,
        symbols::{conversions::scalar::ToScalar, var::Var, variable::Variable},
    },
};
use std::ops::Index;

impl<S: ToScalar> Index<S> for Variable<1, Building> {
    type Output = Var<Building>;
    fn index(&self, indices: S) -> &Self::Output {
        let scalar_indices = [indices.to_scalar_from_host(self.sym_ref())];
        new_var(self, scalar_indices)
    }
}
