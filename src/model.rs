use crate::model_data::ModelData;
use crate::symbols::pars::{Par, ParData};
use crate::symbols::sets::{Set, SetCore, SetData};
use crate::symbols::{DependentSetIndices, Symbol};

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

    pub(crate) fn dep_set<'m, const N: usize>(&'m self, sets: [Set<'m, 0>; N]) -> Set<'m, N> {
        let dep = DependentSetIndices::new(sets.into_iter().map(|x| *x));
        let data = SetData::new(dep);
        self.data.sets.push(self, Symbol::new(data)).with_dim()
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

    pub(crate) fn par<'m, const N: usize>(&'m self, sets: [SetCore<'m>; N]) -> Par<'m, N> {
        let dep = DependentSetIndices::new(sets.into_iter());
        let data = ParData::new(dep);
        self.data.pars.push(self, Symbol::new(data)).with_dim()
    }
}

#[test]
fn abc() {
    use crate::*;

    let m = Model::new();
    let i = m.set();
    let j = set_of([i]);

    let x = m.par([i.core(), j.core()]);
    let x = m.par([i.into(), j.into()]);
    let x = m.par([*i, *j]);
}
