use crate::modeling::model_data::ModelData;
use crate::modeling::reference::HasRef;
use crate::modeling::stages::{Building, Built, ModelStages};
use crate::modeling::symbol_collections::symbol_data::SymbolData;
use crate::{
    data_structs::current_set_indices::CurrentSetElements,
    modeling::symbols::scalar::Scalar,
    numerics::num::{Num, RealNum},
};
use core::panic;
use orx_imp_vec::prelude::NotSelfRefVecItem;
use std::fmt::Debug;
use std::rc::Rc;

pub(crate) type FunGetScalarValue = Rc<dyn Fn(&CurrentSetElements) -> RealNum>;

const STR_ZERO: &str = "0";
const STR_ONE: &str = "1";
const STR_MINUSONE: &str = "-1";

// todo: this fn will be pub(crate)
#[derive(Clone)]
pub enum ScalarVariant {
    Zero,
    One,
    MinusOne,
    Real { str: String, num: RealNum },
    Func { str: String, fun: FunGetScalarValue },
}
impl ScalarVariant {
    pub(crate) fn new_from_num(num: RealNum) -> Self {
        if num.is_zero() {
            ScalarVariant::Zero
        } else if num.is_one() {
            ScalarVariant::One
        } else if num.is_minus_one() {
            ScalarVariant::MinusOne
        } else {
            ScalarVariant::Real {
                str: num.to_string(),
                num,
            }
        }
    }

    pub(crate) fn as_resolved(&self) -> ResolvedScalarVariant {
        use ScalarVariant::*;
        match self {
            Zero => ResolvedScalarVariant::Real { str: "0", num: 0.0 },
            One => ResolvedScalarVariant::Real { str: "1", num: 1.0 },
            MinusOne => ResolvedScalarVariant::Real {
                str: "-1",
                num: -1.0,
            },
            Real { str, num } => ResolvedScalarVariant::Real { str, num: *num },
            Func { str, fun } => ResolvedScalarVariant::Func {
                str,
                fun: fun.clone(),
            },
        }
    }
    pub(crate) fn str(&self) -> &str {
        use ScalarVariant::*;
        match self {
            Zero => STR_ZERO,
            One => STR_ONE,
            MinusOne => STR_MINUSONE,
            Real { str, num: _ } => str,
            Func { str, fun: _ } => str,
        }
    }

    // todo: this fn will be pub(crate)
    pub fn get_value(&self, indices: &CurrentSetElements) -> RealNum {
        use ScalarVariant::*;
        match self {
            Zero => 0.0,
            One => 1.0,
            MinusOne => -1.0,
            Real { str: _, num } => *num,
            Func { str: _, fun } => fun(indices),
        }
    }
    pub(crate) fn get_value_as_index(&self, indices: &CurrentSetElements) -> usize {
        use ScalarVariant::*;
        match self {
            Zero => 0,
            One => 1,
            MinusOne => panic!("nono!!!"),
            Real { str: _, num } => Self::real_to_index(*num),
            Func { str: _, fun } => Self::real_to_index(fun(indices)),
        }
    }

    fn real_to_index(real: RealNum) -> usize {
        // todo! proper conversion
        real as usize
    }
}
impl Debug for ScalarVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zero => write!(f, "Zero"),
            Self::One => write!(f, "One"),
            Self::MinusOne => write!(f, "MinusOne"),
            Self::Real { str, num } => f
                .debug_struct("Real")
                .field("str", str)
                .field("num", num)
                .finish(),
            Self::Func { str, fun: _ } => f.debug_struct("Func").field("str", str).finish(),
        }
    }
}

pub(crate) enum ResolvedScalarVariant<'a> {
    Real {
        str: &'a str,
        num: RealNum,
    },
    Func {
        str: &'a str,
        fun: FunGetScalarValue,
    },
}

#[derive(Clone, Debug)]
pub(crate) struct ScalarData<B: ModelStages> {
    pub(crate) symbol: Scalar<B>,
    pub(crate) variant: ScalarVariant,
}
impl<B: ModelStages> SymbolData for ScalarData<B> {
    type Symbol = Scalar<B>;
    fn symbol(&self) -> Self::Symbol {
        self.symbol.clone()
    }
}
impl<B: ModelStages> NotSelfRefVecItem for Scalar<B> {}

impl ScalarData<Building> {
    pub(crate) fn build(&self, model: &Rc<ModelData<Built>>) -> ScalarData<Built> {
        ScalarData {
            symbol: Scalar::new(self.symbol.sym_ref().build(model.clone())),
            variant: self.variant.clone(),
        }
    }
}
