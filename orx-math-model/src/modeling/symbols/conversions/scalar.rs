use crate::data_structs::current_set_indices::CurrentSetElements;
use crate::modeling::model_data_ref::ModelDataRef;
use crate::modeling::ops::scalar::core::new_scalar;
use crate::modeling::stages::Building;
use crate::modeling::symbol_collections::symbol_collection::SymbolCollection;
use crate::{
    modeling::{
        reference::{HasRef, SymRef},
        symbols::{
            data::scalar_data::{ScalarData, ScalarVariant},
            parameter::Parameter,
            scalar::Scalar,
            set::Set,
        },
    },
    numerics::num::Num,
};
use std::rc::Rc;

// AsScalar
pub(crate) trait AsScalar {
    fn as_scalar(&self) -> Scalar<Building>;
}

impl AsScalar for Scalar<Building> {
    fn as_scalar(&self) -> Scalar<Building> {
        *self
    }
}
impl AsScalar for Set<Building> {
    fn as_scalar(&self) -> Scalar<Building> {
        let core = self.core();
        let scalar_idx = core.sets.get(self.sym_idx()).scalar_ref.sym_idx;
        let scalar_storage = core.scalars.get(scalar_idx);
        scalar_storage.symbol
    }
}
impl AsScalar for Parameter<0, Building> {
    fn as_scalar(&self) -> Scalar<Building> {
        // todo! should we cache or unimportant since rare
        let data = self.data();

        let fun = data.fun.clone();
        let fun = Rc::new(move |_: &CurrentSetElements| fun.value([]));
        let str = data.key.clone();
        let variant = ScalarVariant::Func { str, fun };

        let par_ref = data.symbol.sym_ref();
        let symref = par_ref.core_ref.new_symref_scalar();
        let symbol = Scalar::new(symref);
        let storage = ScalarData { symbol, variant };
        par_ref.core().scalars.push(storage)
    }
}

// ToScalar
pub(crate) trait ToScalar {
    fn to_scalar(&self, core_ref: ModelDataRef<Building>) -> Scalar<Building>;
    fn to_scalar_from_host(&self, host: &SymRef<Building>) -> Scalar<Building> {
        self.to_scalar(host.core_ref)
    }
}

impl<AsSca: AsScalar> ToScalar for AsSca {
    fn to_scalar(&self, _: ModelDataRef<Building>) -> Scalar<Building> {
        self.as_scalar()
    }
}
macro_rules! impl_num {
    ($num:ty) => {
        impl ToScalar for $num {
            fn to_scalar(&self, core_ref: ModelDataRef<Building>) -> Scalar<Building> {
                let variant = ScalarVariant::new_from_num(self.into_real_num());
                new_scalar(core_ref, variant)
            }
        }
    };
}

impl_num!(i64);
impl_num!(i32);
impl_num!(f64);
impl_num!(f32);
