use crate::symbol::Symbol;

#[derive(Clone, Copy)]
pub struct IntPar0<'a>(pub(crate) Symbol<'a, IntPar0Data>);

pub struct IntPar0Data {
    // resolve
    value: i64,
}

impl IntPar0Data {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn value(&self) -> i64 {
        self.value
    }
}
