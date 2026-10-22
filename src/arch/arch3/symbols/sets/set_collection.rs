use crate::symbols::pars::ParCore;
use crate::{symbols::sets::SetCore, Set};
use crate::{Model, Par};

pub trait SetCollection<'m> {
    type SetsArray: IntoIterator<Item = SetCore<'m>>;

    type Par;

    fn model(&self) -> &'m Model;

    fn to_sets_array(self) -> Self::SetsArray;

    fn par_from_core(core: ParCore<'m>) -> Self::Par;
}

impl<'m, const N1: usize> SetCollection<'m> for Set<'m, N1> {
    type SetsArray = [SetCore<'m>; 1];

    type Par = Par<'m, 1>;

    fn model(&self) -> &'m Model {
        self.symbol().model
    }

    fn to_sets_array(self) -> Self::SetsArray {
        [self.into()]
    }

    fn par_from_core(core: ParCore<'m>) -> Self::Par {
        core.with_dim()
    }
}

impl<'m, const N1: usize, const N2: usize> SetCollection<'m> for (Set<'m, N1>, Set<'m, N2>) {
    type SetsArray = [SetCore<'m>; 2];

    type Par = Par<'m, 2>;

    fn model(&self) -> &'m Model {
        self.0.symbol().model
    }

    fn to_sets_array(self) -> Self::SetsArray {
        [self.0.into(), self.1.into()]
    }

    fn par_from_core(core: ParCore<'m>) -> Self::Par {
        core.with_dim()
    }
}

impl<'m, const N1: usize, const N2: usize, const N3: usize> SetCollection<'m>
    for (Set<'m, N1>, Set<'m, N2>, Set<'m, N3>)
{
    type SetsArray = [SetCore<'m>; 3];

    type Par = Par<'m, 3>;

    fn model(&self) -> &'m Model {
        self.0.symbol().model
    }

    fn to_sets_array(self) -> Self::SetsArray {
        [self.0.into(), self.1.into(), self.2.into()]
    }

    fn par_from_core(core: ParCore<'m>) -> Self::Par {
        core.with_dim()
    }
}

impl<'m, const N1: usize, const N2: usize, const N3: usize, const N4: usize> SetCollection<'m>
    for (Set<'m, N1>, Set<'m, N2>, Set<'m, N3>, Set<'m, N4>)
{
    type SetsArray = [SetCore<'m>; 4];

    type Par = Par<'m, 4>;

    fn model(&self) -> &'m Model {
        self.0.symbol().model
    }

    fn to_sets_array(self) -> Self::SetsArray {
        [self.0.into(), self.1.into(), self.2.into(), self.3.into()]
    }

    fn par_from_core(core: ParCore<'m>) -> Self::Par {
        core.with_dim()
    }
}
