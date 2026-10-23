use crate::{symbols::Set, Model};

#[derive(Clone, Copy)]
pub struct SetRef {
    i: usize,
}

impl SetRef {
    pub fn new<'m, 'd>(set: Set<'m, 'd>) -> Self {
        let i = set.el.position_in(set.m.sets()).expect("TODO: error msg");
        Self { i }
    }

    pub fn set<'m, 'd>(self, m: &'m Model<'d>) -> Set<'m, 'd> {
        Set::new(m, m.sets().get(self.i).expect("TODO: error msg"))
    }
}
