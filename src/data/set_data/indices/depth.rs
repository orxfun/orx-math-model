#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Depth {
    d: usize,
}

impl Depth {
    pub fn zero() -> Self {
        Self { d: 0 }
    }

    pub fn next(self) -> Self {
        Self { d: self.d + 1 }
    }

    pub(super) fn new(d: usize) -> Self {
        Self { d }
    }

    pub(super) fn depth(&self) -> usize {
        self.d
    }
}
