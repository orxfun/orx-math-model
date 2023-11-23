use super::symbol_collection::SymbolCollection;
use crate::{
    modeling::stages::{Building, Built, ModelStages},
    modeling::{
        model_data::ModelData,
        model_data_ref::ModelDataRef,
        reference::{HasRef, SymRef},
        symbols::{
            data::scalar_data::{ScalarData, ScalarVariant},
            scalar::Scalar,
        },
    },
};
use orx_imp_vec::ImpVec;
use std::rc::Rc;

const IDX_ONE: usize = 0;
const IDX_MINUSONE: usize = 1;
const IDX_ZERO: usize = 2;

#[derive(Default)]
pub struct Scalars<B: ModelStages> {
    storage: ImpVec<ScalarData<B>>,
}
impl<B: ModelStages> SymbolCollection for Scalars<B> {
    type Storage = ScalarData<B>;
    type Symbol = Scalar<B>;
    fn storage(&self) -> &ImpVec<Self::Storage> {
        &self.storage
    }
}

impl<B: ModelStages> Scalars<B> {
    pub(crate) fn new() -> Self {
        let storage = ImpVec::default();
        Self { storage }
    }
    pub(crate) fn iter_debug(&self) -> impl Iterator<Item = String> + '_ {
        self.storage.iter().map(|x| format!("{:?}", x.symbol))
    }
    pub(crate) fn all_model_references(&self) -> impl Iterator<Item = &SymRef<B>> {
        self.storage.iter().map(|x| x.symbol.sym_ref())
    }
}

impl Scalars<Building> {
    pub(crate) fn add_common_constants(&self, core_ref: ModelDataRef<Building>) {
        Self::push_new_scalar(core_ref, ScalarVariant::One);
        Self::push_new_scalar(core_ref, ScalarVariant::MinusOne);
        Self::push_new_scalar(core_ref, ScalarVariant::Zero);

        assert!(matches!(self.one().data().variant, ScalarVariant::One));
        assert!(matches!(
            self.minus_one().data().variant,
            ScalarVariant::MinusOne
        ));
        assert!(matches!(self.zero().data().variant, ScalarVariant::Zero));
    }
    pub(crate) fn push_new_scalar(
        core_ref: ModelDataRef<Building>,
        variant: ScalarVariant,
    ) -> Scalar<Building> {
        let symref = core_ref.new_symref_scalar();
        let symbol = Scalar::new(symref);
        let storage = ScalarData { symbol, variant };
        core_ref.core().scalars.push(storage)
    }
    pub(crate) fn one(&self) -> Scalar<Building> {
        self.storage[IDX_ONE].symbol
    }
    pub(crate) fn minus_one(&self) -> Scalar<Building> {
        self.storage[IDX_MINUSONE].symbol
    }
    pub(crate) fn zero(&self) -> Scalar<Building> {
        self.storage[IDX_ZERO].symbol
    }

    pub(crate) fn add_to_built_model(&self, model: &Rc<ModelData<Built>>) {
        for d in &self.storage {
            model.scalars.storage.push(d.build(model));
        }
    }
}
