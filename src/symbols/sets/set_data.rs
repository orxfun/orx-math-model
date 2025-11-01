use alloc::vec::Vec;
use core::fmt::Debug;

pub struct SetData {
    depends_on: Vec<usize>,
}

impl SetData {
    pub fn new(depends_on: impl IntoIterator<Item = usize>) -> Self {
        let mut depends_on: Vec<_> = depends_on.into_iter().collect();
        depends_on.sort();
        Self { depends_on }
    }

    pub fn depends_on_indices(&self) -> &[usize] {
        &self.depends_on
    }
}

impl Debug for SetData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SetData")
            .field("depends_on", &self.depends_on)
            .finish()
    }
}
