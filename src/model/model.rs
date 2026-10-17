use crate::model::symbols::ParD0;
use crate::model::{Set0Gen, Set1Gen, Slots};

pub struct Model {
    int: Slots<i64>,
    set0: Slots<Set0Gen>,
    set1: Slots<Set1Gen>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            set0: Slots::new(),
            set1: Slots::new(),
            int: Slots::new(),
        }
    }

    // pars

    pub fn par_d0(&self) -> ParD0<'_> {
        ParD0::new(self.int.push_default())
    }
}
