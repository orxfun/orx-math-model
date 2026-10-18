use crate::model::symbols::{Par, Set};
use crate::model::{Set0Gen, Slots};

pub struct Model {
    int: Slots<i64>,
    set0: Slots<Set0Gen>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            set0: Slots::new(),
            int: Slots::new(),
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
}
