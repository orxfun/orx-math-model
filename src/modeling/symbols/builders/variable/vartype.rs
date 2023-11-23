use super::definition::VariableDefinition;
use crate::modeling::modeler::Modeler;
use crate::modeling::stages::Building;
use crate::modeling::symbol_collections::symbol_collection::SymbolCollection;
use crate::modeling::symbol_collections::variables::Variables;
use crate::modeling::symbols::parameter::Parameter;
use crate::modeling::{
    modeler::SymbolFromBuilder,
    reference::HasRef,
    symbols::{
        data::{
            variable::{bounds::Bounds, vartype::VariableType},
            variable_data::VariableData,
        },
        variable::Variable,
    },
};
use crate::numerics::num::Num;

pub struct VariableDefinitionType<const D: usize> {
    pub(crate) definition: VariableDefinition,
    pub(crate) vartype: VariableType,
}

impl<const D: usize> VariableDefinitionType<D> {
    fn push_get_var(
        self,
        vars: &Variables<D, Building>,
        bounds: Bounds<D>,
    ) -> Variable<D, Building> {
        let sym_ref = self
            .definition
            .modeler
            .new_index(SymbolFromBuilder::Variable { dim: D });
        let (key, definition) = self.definition.symbol.into_key_definition();
        let symbol = Variable::new(sym_ref);
        let storage = VariableData {
            symbol,
            bounds,
            vartype: self.vartype,
            key,
            definition,
        };
        vars.push(storage)
    }
    fn modeler(&self) -> Modeler {
        self.definition.modeler.clone()
    }
}

impl VariableDefinitionType<0> {
    fn vars(modeler: &Modeler) -> &Variables<0, Building> {
        &modeler.data.variables.dim0
    }
}
impl VariableDefinitionType<1> {
    fn vars(modeler: &Modeler) -> &Variables<1, Building> {
        &modeler.data.variables.dim1
    }
}
impl VariableDefinitionType<2> {
    fn vars(modeler: &Modeler) -> &Variables<2, Building> {
        &modeler.data.variables.dim2
    }
}
impl VariableDefinitionType<3> {
    fn vars(modeler: &Modeler) -> &Variables<3, Building> {
        &modeler.data.variables.dim3
    }
}
impl VariableDefinitionType<4> {
    fn vars(modeler: &Modeler) -> &Variables<4, Building> {
        &modeler.data.variables.dim4
    }
}

macro_rules! impl_dim {
    ($dim:tt) => {
        impl VariableDefinitionType<$dim> {
            pub fn is_unbounded(self) -> Variable<$dim, Building> {
                let modeler = self.modeler();
                let vars = Self::vars(&modeler);
                self.push_get_var(vars, Bounds::Unbounded)
            }
            pub fn is_nonnegative(self) -> Variable<$dim, Building> {
                let modeler = self.modeler();
                let vars = Self::vars(&modeler);
                self.push_get_var(vars, Bounds::Nonnegative)
            }
            pub fn is_nonpositive(self) -> Variable<$dim, Building> {
                let modeler = self.modeler();
                let vars = Self::vars(&modeler);
                self.push_get_var(vars, Bounds::Nonpositive)
            }
            pub fn is_between_zero_and_one(self) -> Variable<$dim, Building> {
                let modeler = self.modeler();
                let vars = Self::vars(&modeler);
                self.push_get_var(vars, Bounds::BetweenZeroAndOne)
            }

            pub fn is_nonnegative_with_constant_ub<Upper: Num>(
                self,
                upper_bound: Upper,
            ) -> Variable<$dim, Building> {
                let modeler = self.modeler();
                let vars = Self::vars(&modeler);
                self.push_get_var(
                    vars,
                    Bounds::NonnegativeWithConstUB(upper_bound.into_real_num()),
                )
            }
            pub fn is_nonnegative_with_param_ub(
                self,
                upper_bound: Parameter<$dim, Building>,
            ) -> Variable<$dim, Building> {
                let modeler = self.modeler();
                let vars = Self::vars(&modeler);
                let funvec = upper_bound.data().fun.clone();
                let key = upper_bound.data().key.clone();
                self.push_get_var(vars, Bounds::NonnegativeWithParamUB(key, funvec))
            }

            pub fn is_nonpositive_with_constant_lb<Lower: Num>(
                self,
                lower_bound: Lower,
            ) -> Variable<$dim, Building> {
                let modeler = self.modeler();
                let vars = Self::vars(&modeler);
                self.push_get_var(
                    vars,
                    Bounds::NonpositiveWithConstLB(lower_bound.into_real_num()),
                )
            }
            pub fn is_nonpositive_with_param_lb(
                self,
                lower_bound: Parameter<$dim, Building>,
            ) -> Variable<$dim, Building> {
                let modeler = self.modeler();
                let vars = Self::vars(&modeler);
                let funvec = lower_bound.data().fun.clone();
                let key = lower_bound.data().key.clone();
                self.push_get_var(vars, Bounds::NonpositiveWithParamLB(key, funvec))
            }

            pub fn has_constant_bounds<Lower: Num, Upper: Num>(
                self,
                lower_bound: Lower,
                upper_bound: Upper,
            ) -> Variable<$dim, Building> {
                let modeler = self.modeler();
                let vars = Self::vars(&modeler);
                self.push_get_var(
                    vars,
                    Bounds::ConstBounds(lower_bound.into_real_num(), upper_bound.into_real_num()),
                )
            }
            pub fn has_param_bounds(
                self,
                lower_bound: Parameter<$dim, Building>,
                upper_bound: Parameter<$dim, Building>,
            ) -> Variable<$dim, Building> {
                let modeler = self.modeler();
                let vars = Self::vars(&modeler);
                let lower = lower_bound.data().fun.clone();
                let key1 = lower_bound.data().key.clone();
                let upper = upper_bound.data().fun.clone();
                let key2 = upper_bound.data().key.clone();
                self.push_get_var(vars, Bounds::ParamBounds(key1, lower, key2, upper))
            }
        }
    };
}

impl_dim!(0);
impl_dim!(1);
impl_dim!(2);
impl_dim!(3);
impl_dim!(4);
