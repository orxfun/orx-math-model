use crate::{symbols::Set, Model};

pub struct SetRef {
    i: usize,
}

impl SetRef {
    pub fn new<'m>(m: &'m Model, set: Set<'m, '_>) -> Self {
        let i = set.0.position_in(m.sets()).unwrap();
        Self { i }
    }
}
