use super::core::new_var;
use crate::{
    modeling::stages::Building,
    modeling::symbols::{var::Var, variable::Variable},
};
use std::ops::Index;

impl Index<()> for Variable<0, Building> {
    type Output = Var<Building>;
    fn index(&self, _: ()) -> &Self::Output {
        let scalar_indices = [];
        new_var(self, scalar_indices)
    }
}
