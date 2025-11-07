use crate::model_data::ModelData;
use crate::symbols::pars::ParData;
use crate::symbols::sets::{IndependentSetCollection, Set, SetCollection, SetCore, SetData};
use crate::symbols::{DependentSetIndices, Symbol};
use crate::Par;

#[derive(Default)]
pub struct Model {
    pub(crate) data: ModelData,
}

impl Model {
    pub fn new() -> Self {
        Self::default()
    }

    // sets

    pub fn set(&self) -> Set<'_, 0> {
        let dep = DependentSetIndices::new(core::iter::empty());
        let data = SetData::new(dep);
        self.data.sets.push(self, Symbol::new(data)).with_dim()
    }

    pub fn set_of<'m, S>(&'m self, sets: S) -> S::Set
    where
        S: IndependentSetCollection<'m>,
    {
        let sets = sets.to_sets_array();
        let dep = DependentSetIndices::new(sets.into_iter());
        let data = SetData::new(dep);
        let core = self.data.sets.push(self, Symbol::new(data));
        S::set_from_core(core)
    }

    pub fn set_by_key<const N: usize>(&self, key: &str) -> Option<Set<'_, N>> {
        let core = self.data.sets.by_key(self, key).map(SetCore::from);
        core.and_then(|x| x.with_dim_checked::<N>())
    }

    #[inline(always)]
    pub(crate) fn set_at(&self, idx: usize) -> Option<SetCore<'_>> {
        self.data.sets.at(self, idx).map(SetCore::from)
    }

    #[inline(always)]
    pub(crate) fn set_at_unchecked(&self, idx: usize) -> SetCore<'_> {
        self.set_at(idx).expect("must exist")
    }

    // pars

    pub fn scalar(&self) -> Par<'_, 0> {
        let dep = DependentSetIndices::new(core::iter::empty());
        let data = ParData::new(dep);
        self.data.pars.push(self, Symbol::new(data)).with_dim()
    }

    pub fn par<'m, S>(&'m self, sets: S) -> S::Par
    where
        S: SetCollection<'m>,
    {
        let sets = sets.to_sets_array();
        let dep = DependentSetIndices::new(sets.into_iter());
        let data = ParData::new(dep);
        let core = self.data.pars.push(self, Symbol::new(data));
        S::par_from_core(core)
    }
}
