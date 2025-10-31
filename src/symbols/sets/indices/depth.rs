#[derive(Clone, Copy)]
pub struct Depth {
    d: usize,
}

impl Depth {
    pub(super) fn new(d: usize) -> Self {
        Self { d }
    }

    pub(super) fn depth(&self) -> usize {
        self.d
    }
}
