use crate::{symbols::Set, Model};

pub struct SetRef {
    i: usize,
}

impl SetRef {
    pub fn new<'m, 'd>(m: &'m Model<'d>, set: Set<'m, 'd>) -> Self {
        let i = set.0.position_in(m.sets()).expect("TODO: error msg");
        Self { i }
    }
}
