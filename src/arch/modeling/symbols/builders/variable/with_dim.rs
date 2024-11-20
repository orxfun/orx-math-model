use super::{definition::VariableDefinition, vartype::VariableDefinitionType};
use crate::{
    modeling::stages::Building,
    modeling::symbols::{data::variable::vartype::VariableType, variable::Variable},
};

pub struct VariableDefinitionDim<const D: usize> {
    pub(crate) definition: VariableDefinition,
}

impl<const D: usize> VariableDefinitionDim<D> {
    pub fn is_continuous(self) -> VariableDefinitionType<D> {
        VariableDefinitionType {
            definition: self.definition,
            vartype: VariableType::Continuous,
        }
    }

    pub fn is_integer(self) -> VariableDefinitionType<D> {
        VariableDefinitionType {
            definition: self.definition,
            vartype: VariableType::Integer,
        }
    }
}

macro_rules! impl_dim {
    ($dim:tt) => {
        impl VariableDefinitionDim<$dim> {
            pub fn is_binary(self) -> Variable<$dim, Building> {
                VariableDefinitionType::<$dim> {
                    definition: self.definition,
                    vartype: VariableType::Binary,
                }
                .is_between_zero_and_one()
            }
        }
    };
}
impl_dim!(0);
impl_dim!(1);
impl_dim!(2);
impl_dim!(3);
impl_dim!(4);
