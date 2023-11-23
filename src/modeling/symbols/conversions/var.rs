use crate::{
    modeling::stages::Building,
    modeling::symbols::{var::Var, variable::Variable},
};

pub(crate) trait ToVar {
    fn to_var(&self) -> Var<Building>;
}

impl ToVar for Var<Building> {
    fn to_var(&self) -> Var<Building> {
        *self
    }
}

impl ToVar for Variable<0, Building> {
    fn to_var(&self) -> Var<Building> {
        self[()]
    }
}
