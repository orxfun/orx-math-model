use crate::model::{Set0Gen, Slots};

pub struct Model {
    set0: Slots<Set0Gen>,
    int: Slots<i64>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            set0: Slots::new(),
            int: Slots::new(),
        }
    }
}
