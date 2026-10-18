use crate::model::symbols::{Par, Set, Set2};
use crate::model::{Set0Gen, Set0Gen2, Slots};

pub struct Model<'d> {
    int: Slots<i64>,
    set0: Slots<Set0Gen>,
    set0b: Slots<Set0Gen2<'d>>,
}

impl<'d> Model<'d> {
    pub fn new() -> Self {
        Self {
            set0: Slots::new(),
            int: Slots::new(),
            set0b: Slots::new(),
        }
    }

    // pars

    pub fn par_d0(&self) -> Par<'_> {
        Par::new(self.int.push_default())
    }

    // sets

    pub fn set(&self) -> Set<'_> {
        Set::new(self.set0.push_default())
    }

    pub fn set2(&self) -> Set2<'_, 'd> {
        Set2::new(self.set0b.push_default())
    }
}
