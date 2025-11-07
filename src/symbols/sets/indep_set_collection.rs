use crate::Model;
use crate::{symbols::sets::SetCore, Set};

pub trait IndependentSetCollection<'m> {
    type SetsArray: IntoIterator<Item = SetCore<'m>>;

    type Set;

    fn model(&self) -> &'m Model;

    fn to_sets_array(self) -> Self::SetsArray;
}

impl<'m> IndependentSetCollection<'m> for Set<'m, 0> {
    type SetsArray = [SetCore<'m>; 1];

    type Set = Set<'m, 1>;

    fn model(&self) -> &'m Model {
        self.symbol().model
    }

    fn to_sets_array(self) -> Self::SetsArray {
        [self.into()]
    }
}

impl<'m> IndependentSetCollection<'m> for (Set<'m, 0>, Set<'m, 0>) {
    type SetsArray = [SetCore<'m>; 2];

    type Set = Set<'m, 2>;

    fn model(&self) -> &'m Model {
        self.0.symbol().model
    }

    fn to_sets_array(self) -> Self::SetsArray {
        [self.0.into(), self.1.into()]
    }
}

impl<'m> IndependentSetCollection<'m> for (Set<'m, 0>, Set<'m, 0>, Set<'m, 0>) {
    type SetsArray = [SetCore<'m>; 3];

    type Set = Set<'m, 3>;

    fn model(&self) -> &'m Model {
        self.0.symbol().model
    }

    fn to_sets_array(self) -> Self::SetsArray {
        [self.0.into(), self.1.into(), self.2.into()]
    }
}

impl<'m> IndependentSetCollection<'m> for (Set<'m, 0>, Set<'m, 0>, Set<'m, 0>, Set<'m, 0>) {
    type SetsArray = [SetCore<'m>; 4];

    type Set = Set<'m, 4>;

    fn model(&self) -> &'m Model {
        self.0.symbol().model
    }

    fn to_sets_array(self) -> Self::SetsArray {
        [self.0.into(), self.1.into(), self.2.into(), self.3.into()]
    }
}
