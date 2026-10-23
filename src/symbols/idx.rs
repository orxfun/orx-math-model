use crate::model_references::SetRef;
use crate::symbols::Set;
use crate::Model;

pub struct IdxData {
    s0: SetRef,
    value: usize,
}

impl IdxData {
    pub fn default<'m, 'd>(m: &'m Model<'d>, s0: Set<'m, 'd>) -> Self {
        let s0 = SetRef::new(s0);
        Self { s0, value: 0 }
    }
}

pub struct Idx;
