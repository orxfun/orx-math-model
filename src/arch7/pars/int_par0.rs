use crate::symbol::Symbol;

pub struct IntPar0Data {
    // resolve
    value: i64,
}

impl IntPar0Data {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    #[inline(always)]
    pub fn value(&self) -> i64 {
        self.value
    }
}
