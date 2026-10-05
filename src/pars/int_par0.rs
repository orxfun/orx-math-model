use crate::symbol::Symbol;

#[derive(Clone, Copy)]
pub struct IntPar0<'a>(pub(crate) Symbol<'a, IntPar0Data>);

pub struct IntPar0Data {
    // resolve
    pub value: i64,
}

impl IntPar0Data {
    pub fn new() -> Self {
        Self { value: 0 }
    }
}
