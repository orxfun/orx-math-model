#[derive(Clone, Copy)]
pub struct Depth {
    d: usize,
}

impl Depth {
    pub(super) fn depth(&self) -> usize {
        self.d
    }
}
