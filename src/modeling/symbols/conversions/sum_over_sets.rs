use crate::{
    data_structs::sum_over_set_refs::SumOverSetRefs,
    modeling::stages::Building,
    modeling::{model_data_ref::ModelDataRef, reference::HasRef, symbols::set::Set},
};

pub(crate) trait ToSumOverSetIndices {
    fn sum_over_sets(&self) -> SumOverSetRefs<Building>;
    fn core_ref(&self) -> ModelDataRef<Building>;
}

impl ToSumOverSetIndices for Set<Building> {
    fn sum_over_sets(&self) -> SumOverSetRefs<Building> {
        [*self.sym_ref()].into()
    }
    fn core_ref(&self) -> ModelDataRef<Building> {
        self.sym_ref().core_ref
    }
}

impl ToSumOverSetIndices for (Set<Building>, Set<Building>) {
    fn sum_over_sets(&self) -> SumOverSetRefs<Building> {
        [*self.0.sym_ref(), *self.1.sym_ref()].into()
    }
    fn core_ref(&self) -> ModelDataRef<Building> {
        self.0.sym_ref().core_ref
    }
}

impl ToSumOverSetIndices for (Set<Building>, Set<Building>, Set<Building>) {
    fn sum_over_sets(&self) -> SumOverSetRefs<Building> {
        [*self.0.sym_ref(), *self.1.sym_ref(), *self.2.sym_ref()].into()
    }
    fn core_ref(&self) -> ModelDataRef<Building> {
        self.0.sym_ref().core_ref
    }
}

impl ToSumOverSetIndices for (Set<Building>, Set<Building>, Set<Building>, Set<Building>) {
    fn sum_over_sets(&self) -> SumOverSetRefs<Building> {
        [
            *self.0.sym_ref(),
            *self.1.sym_ref(),
            *self.2.sym_ref(),
            *self.3.sym_ref(),
        ]
        .into()
    }
    fn core_ref(&self) -> ModelDataRef<Building> {
        self.0.sym_ref().core_ref
    }
}
